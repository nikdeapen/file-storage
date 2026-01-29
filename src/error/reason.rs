use std::fmt::{Display, Formatter};

/// The reason for a path operation error.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Reason {
    /// The file system could not be resolved for the path.
    UnknownFileSystem,

    /// The file system does not support the operation.
    UnsupportedOperation,

    /// The file was not found.
    FileNotFound,

    /// The file already exists.
    FileAlreadyExists,

    /// The file content was not properly encoded UTF-8.
    FileContentNotUTF8,

    /// Another uncategorized reason.
    // todo -- remove this and have Option<Reason> in the error?
    Other,
}

impl Display for Reason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Reason::UnknownFileSystem => "unknown file system",
            Reason::UnsupportedOperation => "unsupported operation",
            Reason::FileNotFound => "file not found",
            Reason::FileAlreadyExists => "file already exists",
            Reason::FileContentNotUTF8 => "file content not utf-8",
            Reason::Other => "other",
        };
        write!(f, "{}", s)
    }
}
