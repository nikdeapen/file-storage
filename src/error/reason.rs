use std::fmt::{Display, Formatter};

/// The reason for a file or folder operation error.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Reason {
    /// The file system for the path could not be resolved.
    UnknownFileSystem,

    /// The file system does not support the operation.
    OperationNotSupported,

    /// The file was not found. (this can also mean it unauthorized)
    FileNotFound,

    /// The file already exists.
    FileAlreadyExists,

    /// Another uncategorized error.
    Other,
}

impl Display for Reason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
