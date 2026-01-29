use crate::StoragePath;
use crate::{Operation, Reason};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;
use std::path::Path;

/// A storage path operation error.
#[derive(Debug)]
pub struct Error {
    path: StoragePath,
    operation: Operation,
    reason: Reason,
    source: Option<io::Error>,
}

impl Error {
    //! Construction

    /// Creates an error from the `reason`.
    ///
    /// The `source` will be `None`.
    pub fn new<P>(path: P, operation: Operation, reason: Reason) -> Self
    where
        P: Into<StoragePath>,
    {
        let path: StoragePath = path.into();
        Self {
            path,
            operation,
            reason,
            source: None,
        }
    }

    /// Creates an error from the `source`.
    ///
    /// The `reason` will be `Other`.
    pub fn from_source<P, E>(path: P, operation: Operation, source: E) -> Self
    where
        P: Into<StoragePath>,
        E: Into<io::Error>,
    {
        let path: StoragePath = path.into();
        Self {
            path,
            operation,
            reason: Reason::Other,
            source: Some(source.into()),
        }
    }

    /// Creates an error from the `message`.
    ///
    /// The `source` will be a `std::io::Error` with the `message`.
    /// The `reason` will be `Other`.
    pub fn from_message<P, S>(path: P, operation: Operation, message: S) -> Self
    where
        P: Into<StoragePath>,
        S: Into<String>,
    {
        Error::from_source(path, operation, io::Error::other(message.into()))
    }
}

impl Error {
    //! Common IO Errors

    /// Creates the `unknown file system` error.
    pub fn unknown_file_system(path: &str) -> io::Error {
        io::Error::other(format!("unknown file system for path: {}", path))
    }

    /// Creates the `path not utf-8` error.
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

    /// Exports the path.
    pub fn export_path(self) -> StoragePath {
        self.path
    }

    /// Gets the operation.
    pub fn operation(&self) -> Operation {
        self.operation
    }

    /// Gets the reason.
    pub fn reason(&self) -> Reason {
        self.reason
    }

    /// Gets the optional source.
    pub fn source(&self) -> Option<&io::Error> {
        self.source.as_ref()
    }
}

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        let kind: ErrorKind = match error.reason() {
            Reason::UnknownFileSystem => ErrorKind::Unsupported,
            Reason::UnsupportedOperation => ErrorKind::Unsupported,
            Reason::FileNotFound => ErrorKind::NotFound,
            Reason::FileAlreadyExists => ErrorKind::AlreadyExists,
            Reason::FileContentNotUTF8 => ErrorKind::InvalidData,
            Reason::Other => ErrorKind::Other,
        };
        Self::new(kind, error)
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
        if let Some(source) = self.source() {
            write!(f, "source={}", source)?;
        } else {
            write!(f, "reason={}", self.reason)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}
