/// The reason for a file or folder operation error.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Reason {
    /// The file system for the path could not be resolved.
    UnknownFileSystem,

    /// The file was not found. (this can mean it doesn't exist or is not authorized)
    FileNotFound,

    /// The file already exists.
    FileAlreadyExists,

    /// Another uncategorized error.
    Other,
}
