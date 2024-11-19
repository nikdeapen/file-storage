use std::fmt::{Display, Formatter};

/// A file or folder operation.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Operation {
    /// Check if a file exists.
    Exists,

    /// Read a file.
    Read,

    /// Write a file.
    Write,

    /// Delete a file.
    Delete,

    /// List the files in a folder.
    ListFiles,

    /// Delete the files in a folder.
    DeleteFiles,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
