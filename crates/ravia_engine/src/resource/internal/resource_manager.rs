use std::{
    collections::HashMap,
    io::Read,
    sync::{mpsc, Arc, Mutex},
};

use super::{
    error::{Error, Result},
    resource::Resource,
};

#[derive(Debug)]
enum ResourceRequest {
    Load(Resource),
}

/// The state of a resource.
#[derive(Debug, Clone)]
pub enum ResourceState {
    Loading,
    Loaded(Vec<u8>),
    Error(Error),
}

/// A unique key to access the actual resource from the [`ResourceManager`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceKey(u64);

/// Resource manager handles loading external resources from filesystem or the web
/// and caching them for reuse.=
pub struct ResourceManager {
    request_tx: mpsc::Sender<ResourceRequest>,
    resource_key_counter: Mutex<u64>,
    store: Arc<Mutex<HashMap<ResourceKey, ResourceState>>>,

    #[cfg(not(target_arch = "wasm32"))]
    runtime: tokio::runtime::Runtime,
}

impl std::fmt::Debug for ResourceManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceManager")
    }
}

impl ResourceManager {
    /// Creates a new [`ResourceManager`].
    pub fn new() -> Self {
        let (request_tx, request_rx) = mpsc::channel::<ResourceRequest>();
        let store = Arc::new(Mutex::new(HashMap::new()));

        // spawn a thread to handle resource requests.
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                while let Ok(request) = request_rx.recv() {
                    match request {
                        ResourceRequest::Load(res) => {
                            let _result = Self::load(&res).await;
                        }
                    }
                }
            });

            Self {
                request_tx,
                resource_key_counter: Mutex::new(0),
                store,
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to build async runtime");

            {
                let store = store.clone();
                runtime.spawn(async move {
                    while let Ok(request) = request_rx.recv() {
                        match request {
                            ResourceRequest::Load(res) => {
                                let result = Self::load(&res).await;
                                let key = res.key.unwrap();
                                let mut store = store.lock().unwrap();
                                match result {
                                    Ok(data) => {
                                        store.insert(key, ResourceState::Loaded(data));
                                    }
                                    Err(e) => {
                                        store.insert(key, ResourceState::Error(e));
                                    }
                                }
                            }
                        }
                    }
                });
            }

            Self {
                request_tx,
                resource_key_counter: Mutex::new(0),
                store,
                runtime,
            }
        }
    }

    /// Requests a resource to be loaded.
    pub fn request(&self, res: &mut Resource) {
        log::info!("requesting resource: {:?}", res);

        let key = self.issue_key();
        res.key = Some(key);

        self.store
            .lock()
            .unwrap()
            .insert(key, ResourceState::Loading);

        self.request_tx
            .send(ResourceRequest::Load(res.clone()))
            .expect("failed to send resource request");
    }

    pub fn get(&self, key: ResourceKey) -> ResourceState {
        let store = self.store.lock().unwrap();
        if let Some(state) = store.get(&key) {
            state.clone()
        } else {
            ResourceState::Error(Error::Unknown)
        }
    }

    fn issue_key(&self) -> ResourceKey {
        let mut counter = self.resource_key_counter.lock().unwrap();
        let key = ResourceKey(*counter);
        *counter += 1;
        key
    }

    /// Loads resource and provide it as an [`std::io::Read`] stream.
    async fn load(res: &Resource) -> Result<Vec<u8>> {
        #[cfg(target_arch = "wasm32")]
        {
            todo!()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::load_from_filesystem(res).await
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn load_from_filesystem(res: &Resource) -> Result<Vec<u8>> {
        log::info!("loading resource from filesystem: {:?}", res);

        let resource_root = std::env::var("RAVIA_RES").expect("RAVIA_RES is not set");
        let resource_root = std::path::PathBuf::from(&resource_root);

        let path = resource_root.join(&res.path);
        match std::fs::File::open(path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                Ok(buffer)
            }
            Err(_) => Err(Error::NotFound(res.clone())),
        }
    }
}
