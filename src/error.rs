use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum ServiceLocatorError {
    NotProvided,
    AlreadyProvided,
    /// Some caller panicked while the service was mutably borrowed, see [`std::sync::RwLock::read`] and [`std::sync::RwLock::write`].
    Poisoned,
}

impl Error for ServiceLocatorError {}

impl Display for ServiceLocatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceLocatorError::NotProvided =>
                write!(f, "The service is not provided yet."),
            ServiceLocatorError::AlreadyProvided =>
                write!(f, "The service is already provided."),
            ServiceLocatorError::Poisoned =>
                write!(f, "Some caller panicked while the service was mutably borrowed."),
        }
    }
}

impl From<ServiceLocatorError> for io::Error {
    fn from(error: ServiceLocatorError) -> io::Error {
        let error_kind = match error {
            ServiceLocatorError::NotProvided => io::ErrorKind::NotFound,
            ServiceLocatorError::AlreadyProvided => io::ErrorKind::AlreadyExists,
            ServiceLocatorError::Poisoned => io::ErrorKind::Other,
        };

        io::Error::new(error_kind, error.to_string())
    }
}
