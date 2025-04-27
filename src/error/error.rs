use crate::Reason::Other;
use crate::StoragePath;
use crate::{Operation, Reason};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;
use std::io::ErrorKind::{AlreadyExists, NotFound, Unsupported};
use std::path::Path;

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

    /// Creates an error from the `reason`.
    ///
    /// The `cause` will be `None`.
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
    ///
    /// The `reason` will be `Other`.
    pub fn from_cause<P, E>(path: P, operation: Operation, cause: E) -> Self
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
    ///
    /// The `cause` will be an `std::io::Error` with the `message`.
    /// The `reason` will be `Other`.
    pub fn from_message<P, S>(path: P, operation: Operation, message: S) -> Self
    where
        P: Into<StoragePath>,
        S: Into<String>,
    {
        Error::from_cause(
            path,
            operation,
            io::Error::new(ErrorKind::Other, message.into()),
        )
    }
}

impl Error {
    //! Common IO Errors

    /// Gets the `unknown file system` error.
    pub fn unknown_file_system(path: &str) -> io::Error {
        io::Error::new(
            ErrorKind::Other,
            format!("unknown file system for path: {}", path),
        )
    }

    /// Gets the `path not utf-8` error.
    pub fn path_not_utf8(path: &Path) -> io::Error {
        io::Error::new(
            ErrorKind::InvalidInput,
            format!("path not utf-8: {:?}", path),
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

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        let kind: ErrorKind = match error.reason() {
            Reason::UnknownFileSystem => Unsupported,
            Reason::UnsupportedOperation => Unsupported,
            Reason::FileNotFound => NotFound,
            Reason::FileAlreadyExists => AlreadyExists,
            Other => ErrorKind::Other,
        };
        io::Error::new(kind, error)
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
