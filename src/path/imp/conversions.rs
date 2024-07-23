use crate::path::{FilePath, FolderPath, Path};

impl Path {
    //! Conversions

    /// Converts the path to a folder path. Returns `None` if the path is not a folder path.
    pub fn to_folder(self) -> Option<FolderPath> {
        if self.is_folder() {
            Some(unsafe { FolderPath::new_unchecked(self) })
        } else {
            None
        }
    }

    /// Converts the path to a file path. Returns `None` if the path is not a file path.
    pub fn to_file(self) -> Option<FilePath> {
        if self.is_file() {
            Some(unsafe { FilePath::new_unchecked(self) })
        } else {
            None
        }
    }
}

impl From<FilePath> for Path {
    fn from(file_path: FilePath) -> Self {
        file_path.to_path()
    }
}

impl From<&FilePath> for Path {
    fn from(file_path: &FilePath) -> Self {
        file_path.clone().to_path()
    }
}

impl From<FolderPath> for Path {
    fn from(folder_path: FolderPath) -> Self {
        folder_path.to_path()
    }
}

impl From<&FolderPath> for Path {
    fn from(folder_path: &FolderPath) -> Self {
        folder_path.clone().to_path()
    }
}
