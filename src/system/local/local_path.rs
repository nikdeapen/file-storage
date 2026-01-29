use crate::StoragePath;

/// A local storage path.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct LocalPath<'a> {
    pub(in crate::system::local) path: &'a StoragePath,
}

impl<'a> LocalPath<'a> {
    //! Construction

    /// Creates a local path from the storage `path`.
    ///
    /// Returns `None` if the `path` is not a local path.
    pub fn from(path: &'a StoragePath) -> Option<Self> {
        if path.is_local_path() {
            Some(Self { path })
        } else {
            None
        }
    }
}
