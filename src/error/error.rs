use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;

use crate::Reason::Other;
use crate::StoragePath;
use crate::{Operation, Reason};

/// A file or folder operation error.
#[derive(Debug)]
pub struct Error {
    path: StoragePath,
    operation: Operation,
    reason: Reason,
    cause: Option<io::Error>,
}

impl Error {
    //! Construction

    /// Creates a new error.
    pub fn new<P>(path: P, operation: Operation, reason: Reason) -> Self
    where
        P: Into<StoragePath>,
    {
        let path: StoragePath = path.into();
        Self {
            path,
            operation,
            reason,
            cause: None,
        }
    }

    /// Creates an error from the `cause`.
    pub fn from_error<P, E>(path: P, operation: Operation, cause: E) -> Self
    where
        P: Into<StoragePath>,
        E: Into<io::Error>,
    {
        let path: StoragePath = path.into();
        Self {
            path,
            operation,
            reason: Other,
            cause: Some(cause.into()),
        }
    }

    /// Creates an error from the `message`.
    pub fn from_message<P, S>(path: P, operation: Operation, message: S) -> Self
    where
        P: Into<StoragePath>,
        S: Into<String>,
    {
        Error::from_error(
            path,
            operation,
            io::Error::new(ErrorKind::Other, message.into()),
        )
    }
}

impl Error {
    //! Errors

    /// Gets the error for non UTF-8 encoded paths.
    pub fn path_not_utf8() -> io::Error {
        io::Error::new(ErrorKind::Unsupported, "path is not UTF-8")
    }

    /// Gets the error for unsupported file systems.
    pub fn unsupported_file_system(path: &str) -> io::Error {
        io::Error::new(
            ErrorKind::Unsupported,
            format!("unsupported file system for path: {}", path),
        )
    }
}

impl Error {
    //! Properties

    /// Gets the path.
    pub fn path(&self) -> &StoragePath {
        &self.path
    }

    /// Gets the operation.
    pub fn operation(&self) -> Operation {
        self.operation
    }

    /// Gets the reason.
    pub fn reason(&self) -> Reason {
        self.reason
    }

    /// Gets the optional cause.
    pub fn cause(&self) -> Option<&io::Error> {
        self.cause.as_ref()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let file_or_folder: &str = if self.path.is_file() {
            "file"
        } else {
            "folder"
        };
        write!(
            f,
            "{} error: op={} path={} ",
            file_or_folder, self.operation, self.path
        )?;
        if let Some(cause) = self.cause() {
            write!(f, "cause={}", cause)?;
        } else {
            write!(f, "reason={}", self.reason)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}
