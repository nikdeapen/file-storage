use std::fmt::{Display, Formatter};

/// A path operation.
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

    /// Modify a path.
    ModifyPath,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Operation::Exists => "exists",
            Operation::Read => "read",
            Operation::Write => "write",
            Operation::Delete => "delete",
            Operation::ListFiles => "list-files",
            Operation::DeleteFiles => "delete-files",
            Operation::ModifyPath => "modify-path",
        };
        write!(f, "{}", s)
    }
}
