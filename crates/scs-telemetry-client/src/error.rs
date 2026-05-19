use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Shared memory object not found — plugin not running
    NotFound,
    /// Failed to map view of file
    MapFailed,
    /// Platform not supported
    Unsupported,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound => f.write_str("SCS telemetry shared memory not found — is the plugin running?"),
            Error::MapFailed => f.write_str("Failed to map shared memory view"),
            Error::Unsupported => f.write_str("Platform not supported"),
        }
    }
}

impl std::error::Error for Error {}
