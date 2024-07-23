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
}

impl AsRef<str> for Operation {
    fn as_ref(&self) -> &str {
        match self {
            Self::Exists => "exists",
            Self::Read => "read",
            Self::Write => "write",
            Self::Delete => "delete",
            Self::ListFiles => "list-files",
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
