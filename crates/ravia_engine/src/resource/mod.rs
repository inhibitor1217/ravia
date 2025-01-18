// implementation module
mod internal;

pub use internal::{
    error::{Error, Result},
    resource::Resource,
    resource_manager::ResourceManager,
};
