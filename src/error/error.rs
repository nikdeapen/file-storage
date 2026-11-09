use crate::{Operation, Reason, StoragePath};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;
use std::path::Path;

/// A storage path operation error.
///
/// # Note
/// - `Display` shows the IO source. `source()` skips it and returns the IO source's own source.
/// - This keeps an error chain from printing the IO source twice. Use `io_source` to get it.
#[derive(Debug)]
pub struct Error {
    path: StoragePath,
    operation: Operation,
    reason: Reason,
    source: Option<io::Error>,
}

impl Error {
    //! Common IO Errors
    //!
    //! # Note
    //! - These return an `io::Error`. They run before there is a `StoragePath` for an `Error`.

    /// Creates the `unknown file system` error.
    pub(crate) fn unknown_file_system(path: &str) -> io::Error {
        io::Error::new(
            ErrorKind::Unsupported,
            format!("unknown file system for path: {}", path),
        )
    }

    /// Creates the `path not utf-8` error.
    pub(crate) fn path_not_utf8(path: &Path) -> io::Error {
        io::Error::new(
            ErrorKind::InvalidInput,
            format!("path not utf-8: {:?}", path),
        )
    }

    /// Creates the `invalid path` error.
    pub(crate) fn invalid_path(path: &str) -> io::Error {
        io::Error::new(ErrorKind::InvalidInput, format!("invalid path: {}", path))
    }
}

impl Error {
    //! Construction

    /// Creates an error from the `reason`.
    ///
    /// # Note
    /// - The source will be `None`.
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
    /// # Note
    /// - The reason will be `Other`.
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
    /// # Note
    /// - The source will be an `io::Error` holding the `message`.
    /// - The reason will be `Other`.
    pub fn from_message<P, S>(path: P, operation: Operation, message: S) -> Self
    where
        P: Into<StoragePath>,
        S: Into<String>,
    {
        Self::from_source(path, operation, io::Error::other(message.into()))
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

    /// Gets the optional IO error source.
    pub fn io_source(&self) -> Option<&io::Error> {
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
            Reason::FileContentNotUtf8 => ErrorKind::InvalidData,
            Reason::InvalidPath => ErrorKind::InvalidInput,
            Reason::Other => error.io_source().map_or(ErrorKind::Other, io::Error::kind),
        };
        Self::new(kind, error)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().and_then(std::error::Error::source)
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
            "{} error: op={} path={}",
            file_or_folder, self.operation, self.path
        )?;
        if let Some(source) = self.io_source() {
            write!(f, " source={}", source)?;
        } else {
            write!(f, " reason={}", self.reason)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Operation, Reason, StoragePath};
    use std::fmt::{Display, Formatter};
    use std::io;
    use std::io::ErrorKind;

    #[derive(Debug)]
    struct Leaf;

    impl Display for Leaf {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("leaf")
        }
    }

    impl std::error::Error for Leaf {}

    #[derive(Debug)]
    struct Wrapper {
        leaf: Leaf,
    }

    impl Display for Wrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("wrapper")
        }
    }

    impl std::error::Error for Wrapper {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.leaf)
        }
    }

    fn file() -> StoragePath {
        StoragePath::parse("/folder/file.txt").unwrap()
    }

    #[test]
    fn into_io_error_kind() {
        for (reason, kind) in [
            (Reason::UnknownFileSystem, ErrorKind::Unsupported),
            (Reason::UnsupportedOperation, ErrorKind::Unsupported),
            (Reason::FileNotFound, ErrorKind::NotFound),
            (Reason::FileAlreadyExists, ErrorKind::AlreadyExists),
            (Reason::FileContentNotUtf8, ErrorKind::InvalidData),
            (Reason::InvalidPath, ErrorKind::InvalidInput),
            (Reason::Other, ErrorKind::Other),
        ] {
            let error: io::Error = Error::new(file(), Operation::Read, reason).into();
            assert_eq!(error.kind(), kind, "{reason}");
        }
    }

    #[test]
    fn into_io_error_keeps_source_kind() {
        let source: io::Error = io::Error::from(ErrorKind::PermissionDenied);
        let error: io::Error = Error::from_source(file(), Operation::Write, source).into();
        assert_eq!(error.kind(), ErrorKind::PermissionDenied);
    }

    #[test]
    fn display() {
        let error: Error = Error::new(file(), Operation::Read, Reason::FileNotFound);
        assert_eq!(
            error.to_string(),
            "file error: op=read path=/folder/file.txt reason=file not found"
        );

        let folder: StoragePath = StoragePath::parse("/folder/").unwrap();
        let error: Error = Error::from_message(folder, Operation::ListFiles, "boom");
        assert_eq!(
            error.to_string(),
            "folder error: op=list-files path=/folder/ source=boom"
        );
    }

    #[test]
    fn source_skips_the_io_source() {
        let source: io::Error = io::Error::other(Wrapper { leaf: Leaf });
        let error: Error = Error::from_source(file(), Operation::Read, source);
        assert_eq!(error.io_source().unwrap().to_string(), "wrapper");
        assert!(error.to_string().ends_with(" source=wrapper"));

        let next: &dyn std::error::Error = std::error::Error::source(&error).unwrap();
        assert_eq!(next.to_string(), "leaf");
    }

    #[test]
    fn source_is_none_without_a_deeper_source() {
        let error: Error = Error::from_message(file(), Operation::Read, "boom");
        assert!(std::error::Error::source(&error).is_none());

        let error: Error = Error::new(file(), Operation::Read, Reason::FileNotFound);
        assert!(std::error::Error::source(&error).is_none());
    }
}
