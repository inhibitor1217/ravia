use std::path::PathBuf;

/// Returns a resolved path for an engine resource.
pub fn engine_resource<S: AsRef<str>>(path: S) -> PathBuf {
    let dir = std::env::var("RAVIA_ENGINE_RES")
        .expect("unsupported platform: RAVIA_ENGINE_RES is not set");
    let mut p = PathBuf::from(dir);
    p.push(path.as_ref());
    p
}

/// Returns a resolved path for a user resource.
pub fn user_resource<S: AsRef<str>>(path: S) -> PathBuf {
    let dir =
        std::env::var("RAVIA_USER_RES").expect("unsupported platform: RAVIA_USER_RES is not set");
    let mut p = PathBuf::from(dir);
    p.push(path.as_ref());
    p
}
