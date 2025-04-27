use std::fmt::{Display, Formatter};

/// The reason for a file or folder operation error.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Reason {
    /// The file system for the path could not be resolved.
    UnknownFileSystem,

    /// The file system does not support the operation.
    UnsupportedOperation,

    /// The file was not found.
    FileNotFound,

    /// The file already exists.
    FileAlreadyExists,

    /// Another uncategorized error.
    Other,
}

impl Display for Reason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Reason::UnknownFileSystem => "unknown file system",
            Reason::UnsupportedOperation => "operation not supported",
            Reason::FileNotFound => "file not found",
            Reason::FileAlreadyExists => "file already exists",
            Reason::Other => "other",
        };
        write!(f, "{}", s)
    }
}
