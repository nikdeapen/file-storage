use crate::Operation::DeleteFiles;
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FolderPath, LocalPath};

impl FolderPath {
    //! Delete Files

    /// Deletes all files in the folder recursively.
    ///
    /// # Note
    /// - Every file beneath the folder is deleted, not just the files directly in it.
    /// - A missing folder has nothing to delete. On Cloudflare R2 a missing bucket is an error.
    /// - It stops at the first error. The files deleted before then stay deleted.
    /// - On a local file system the subfolders it empties are deleted too. The folder itself is
    ///   kept.
    /// - Local symlinks are deleted themselves. Their targets are never touched.
    /// - Local entries that are neither files nor folders, such as sockets, are kept. So are the
    ///   folders that hold them.
    /// - On Cloudflare R2 the folder markers beneath the folder are deleted too. Keys that end
    ///   with `/` are folder markers.
    /// - On Cloudflare R2 the bucket's root folder deletes every object in the bucket.
    pub fn delete_files(&self) -> Result<(), Error> {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.delete_files();
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.delete_files();
        }

        Err(Error::new(self.clone(), DeleteFiles, UnknownFileSystem))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, FolderPath, Operation, Reason, StoragePath};

    #[test]
    fn other_hosts_path_is_unknown_file_system() {
        let path: &str = if cfg!(windows) {
            "/folder/"
        } else {
            "C:\\folder\\"
        };
        let folder: FolderPath = StoragePath::parse(path).unwrap().to_folder().unwrap();
        let error: Error = folder.delete_files().unwrap_err();
        assert_eq!(error.operation(), Operation::DeleteFiles);
        assert_eq!(error.reason(), Reason::UnknownFileSystem);
    }
}
