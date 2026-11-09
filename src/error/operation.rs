use std::fmt::{Display, Formatter};

/// A path operation.
#[must_use]
#[non_exhaustive]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Operation {
    /// Checks if a file exists.
    Exists,

    /// Reads a file.
    Read,

    /// Writes a file.
    Write,

    /// Deletes a file.
    Delete,

    /// Lists the files in a folder.
    ListFiles,

    /// Deletes the files in a folder.
    DeleteFiles,

    /// Converts a path to a file path or a folder path.
    ConvertPath,
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
            Operation::ConvertPath => "convert-path",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::Operation;

    #[test]
    fn display() {
        for (operation, expected) in [
            (Operation::Exists, "exists"),
            (Operation::Read, "read"),
            (Operation::Write, "write"),
            (Operation::Delete, "delete"),
            (Operation::ListFiles, "list-files"),
            (Operation::DeleteFiles, "delete-files"),
            (Operation::ConvertPath, "convert-path"),
        ] {
            assert_eq!(operation.to_string(), expected);
        }
    }
}
