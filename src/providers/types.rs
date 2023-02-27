
pub struct Metadata {
    name: String,
}

pub struct ResolutionError {
    pub code:    String,
    pub message: String
}
pub struct ResolutionDetails<T> {
    pub value: T,
    pub resolution_error: ResolutionError,
    pub reason:          String,
    pub varient:         String
}
