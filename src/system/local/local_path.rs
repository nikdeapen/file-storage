use crate::StoragePath;
use std::fs::FileType;
use std::path::Path;

/// A local storage path.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub(crate) struct LocalPath<'a> {
    pub(in crate::system::local) path: &'a StoragePath,
}

impl<'a> LocalPath<'a> {
    //! Construction

    /// Creates the [LocalPath].
    ///
    /// # Note
    /// - It is `None` if the `path` is not a local path on this host.
    pub(crate) fn new(path: &'a StoragePath) -> Option<Self> {
        if path.is_local_path() {
            Some(Self { path })
        } else {
            None
        }
    }
}

impl<'a> LocalPath<'a> {
    //! Properties

    /// Gets the path for `std::fs`.
    pub(in crate::system::local) fn std_path(self) -> &'a Path {
        Path::new(self.path.as_str())
    }
}

impl<'a> LocalPath<'a> {
    //! Folder Links

    /// Checks if the `file_type` is a Windows symlink to a folder.
    ///
    /// # Note
    /// - Windows deletes these with `remove_dir`. `remove_file` cannot.
    /// - It is `false` on other hosts. Their links are all deleted with `remove_file`.
    pub(in crate::system::local) fn is_folder_link(self, file_type: FileType) -> bool {
        #[cfg(windows)]
        {
            std::os::windows::fs::FileTypeExt::is_symlink_dir(&file_type)
        }
        #[cfg(not(windows))]
        {
            let _ = file_type;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LocalPath, StoragePath};

    #[test]
    fn new_accepts_only_host_paths() {
        let unix: StoragePath = StoragePath::parse("/file.txt").unwrap();
        let windows: StoragePath = StoragePath::parse("C:\\file.txt").unwrap();
        assert_eq!(LocalPath::new(&unix).is_some(), !cfg!(windows));
        assert_eq!(LocalPath::new(&windows).is_some(), cfg!(windows));
    }

    #[test]
    fn std_path() {
        let path: StoragePath = StoragePath::parse(if cfg!(windows) {
            "C:\\folder\\file.txt"
        } else {
            "/folder/file.txt"
        })
        .unwrap();
        let local: LocalPath = LocalPath::new(&path).unwrap();
        assert_eq!(local.std_path().to_str(), Some(path.as_str()));
    }
}
