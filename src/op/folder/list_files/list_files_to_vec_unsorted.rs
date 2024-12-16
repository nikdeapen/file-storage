use crate::path::LocalPath;
use crate::Operation::ListFiles;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath, FolderPath};

impl FolderPath {
    //! List Files to Vec Unsorted

    /// Lists the files to the `target` vec.
    ///
    /// Returns `Ok(file_count)`.
    pub fn list_files_to_vec_unsorted(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        if let Some(local) = LocalPath::parse(self.path()) {
            return local.list_files_to_vec_unsorted(target);
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::r2::R2Path::parse(self.path()) {
            return path.list_files_to_vec(target);
        }

        Err(Error::new(self.clone(), ListFiles, UnknownFileSystem))
    }
}
