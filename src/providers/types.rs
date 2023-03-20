#[derive(Clone)]
pub struct ProviderMetadata {
    pub name: String,
}

pub struct ResolutionError {
    pub code: String,
    pub message: String,
}
pub struct ResolutionDetails<T> {
    pub value: T,
    pub resolution_error: ResolutionError,
    pub reason: String,
    pub variant: String,
}

pub struct Configuration {
    pub host: String,
    pub port: u16,
}

impl Configuration {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
}
