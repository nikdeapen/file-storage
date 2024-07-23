use std::fmt::{Display, Formatter};
use std::io::ErrorKind;

use crate::error::{Operation, Reason};
use crate::Path;
use crate::Reason::Other;

/// A file or folder operation error.
#[derive(Debug)]
pub struct Error {
    path: Path,
    operation: Operation,
    reason: Reason,
    cause: Option<std::io::Error>,
}

impl Error {
    //! Construction

    /// Creates a new error.
    pub fn new<P>(path: P, operation: Operation, reason: Reason) -> Self
    where
        P: Into<Path>,
    {
        let path: Path = path.into();
        Self {
            path,
            operation,
            reason,
            cause: None,
        }
    }

    /// Creates an error from the `std::io::Error`.
    pub fn from_error<P, E>(path: P, operation: Operation, error: E) -> Self
    where
        P: Into<Path>,
        E: Into<std::io::Error>,
    {
        let path: Path = path.into();
        Self {
            path,
            operation,
            reason: Other,
            cause: Some(error.into()),
        }
    }

    /// Creates an error from the message.
    pub fn from_message<P, S>(path: P, operation: Operation, message: S) -> Self
    where
        P: Into<Path>,
        S: Into<String>,
    {
        Error::from_error(
            path,
            operation,
            std::io::Error::new(ErrorKind::Other, message.into()),
        )
    }
}

impl Error {
    //! Properties

    /// Gets the path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Gets the operation.
    pub fn operation(&self) -> Operation {
        self.operation
    }

    /// Gets the error info.
    pub fn info(&self) -> Reason {
        self.reason
    }

    /// Gets the optional cause.
    pub fn cause(&self) -> Option<&std::io::Error> {
        self.cause.as_ref()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ff: &str = if self.path.is_file() {
            "file"
        } else {
            "folder"
        };
        write!(
            f,
            "{} error: op={} path={}, info={:?}",
            ff, self.path, self.operation, self.reason
        )?;
        if let Some(cause) = self.cause() {
            write!(f, " cause={}", cause)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}
