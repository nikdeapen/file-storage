use crate::op::ListFilesOpInner;
use crate::Operation::ListFiles;
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FolderPath, ListFilesOp, LocalPath};

impl FolderPath {
    //! List Files

    /// Creates a list-files operation on the folder.
    ///
    /// # Note
    /// - Every file beneath the folder is listed, not just the files directly in it.
    /// - The files are sorted by the bytes of their paths.
    /// - A missing folder lists nothing. On Cloudflare R2 a missing bucket is an error.
    /// - The listing ends after its first error.
    /// - Cloudflare R2 keys that end with `/` are folder markers. They are skipped.
    /// - Local symlinks are followed. A file under a linked folder is listed under the link's
    ///   path. A link back to a folder being listed is skipped.
    /// - Local entries that are neither files nor folders, such as sockets, are skipped.
    /// - A local file name that is not UTF-8 is an error.
    pub fn list_files(&self) -> Result<ListFilesOp, Error> {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.list_files().map(|op| ListFilesOp {
                inner: ListFilesOpInner::Local(op),
            });
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.list_files().map(|op| ListFilesOp {
                inner: ListFilesOpInner::R2(op),
            });
        }

        Err(Error::new(self.clone(), ListFiles, UnknownFileSystem))
    }

    /// Creates a list-files operation on the folder where the files may be unsorted.
    ///
    /// # Note
    /// - It lists the same files as `list_files`, in any order.
    pub fn list_files_unsorted(&self) -> Result<ListFilesOp, Error> {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.list_files_unsorted().map(|op| ListFilesOp {
                inner: ListFilesOpInner::Local(op),
            });
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.list_files().map(|op| ListFilesOp {
                inner: ListFilesOpInner::R2(op),
            });
        }

        Err(Error::new(self.clone(), ListFiles, UnknownFileSystem))
    }
}
