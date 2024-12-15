use crate::StoragePath;

/// A local path.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct LocalPath<'a> {
    pub(in crate::path::system::local) path: &'a StoragePath,
}

impl<'a> LocalPath<'a> {
    //! Parse

    /// Parses the path.
    ///
    /// Returns `None` if the path was not a local path.
    pub fn parse(path: &'a StoragePath) -> Option<Self> {
        if path.is_local_path() {
            Some(Self { path })
        } else {
            None
        }
    }
}
