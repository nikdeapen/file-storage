use std::fmt::{Display, Formatter};

/// The reason for a path operation error.
#[must_use]
#[non_exhaustive]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Reason {
    /// No backend on this host handles the path, such as a Windows path on Unix.
    UnknownFileSystem,

    /// The backend cannot do the operation, such as an atomic delete on Cloudflare R2.
    UnsupportedOperation,

    /// The file does not exist.
    FileNotFound,

    /// The file already exists.
    FileAlreadyExists,

    /// The file's content is not UTF-8.
    FileContentNotUtf8,

    /// The path is the wrong kind, such as a folder path where a file path is needed.
    InvalidPath,

    /// Any other reason. The IO source holds the details.
    Other,
}

impl Display for Reason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Reason::UnknownFileSystem => "unknown file system",
            Reason::UnsupportedOperation => "unsupported operation",
            Reason::FileNotFound => "file not found",
            Reason::FileAlreadyExists => "file already exists",
            Reason::FileContentNotUtf8 => "file content not utf-8",
            Reason::InvalidPath => "invalid path",
            Reason::Other => "other",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::Reason;

    #[test]
    fn display() {
        for (reason, expected) in [
            (Reason::UnknownFileSystem, "unknown file system"),
            (Reason::UnsupportedOperation, "unsupported operation"),
            (Reason::FileNotFound, "file not found"),
            (Reason::FileAlreadyExists, "file already exists"),
            (Reason::FileContentNotUtf8, "file content not utf-8"),
            (Reason::InvalidPath, "invalid path"),
            (Reason::Other, "other"),
        ] {
            assert_eq!(reason.to_string(), expected);
        }
    }
}
