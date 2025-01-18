use super::resource::Resource;

/// Possible errors for managing resources.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ResourceLoadFailed(Resource),
    ResourceNotFound(Resource),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::ResourceLoadFailed(res) => format!("failed to load resource: {}", res.path),
            Error::ResourceNotFound(res) => format!("resource not found: {}", res.path),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}

/// Result type for resource management.
pub type Result<T> = std::result::Result<T, Error>;
