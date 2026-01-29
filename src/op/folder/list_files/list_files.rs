use crate::op::ListFilesOpInner;
use crate::system::LocalPath;
use crate::Operation::ListFiles;
use crate::Reason::UnsupportedOperation;
use crate::{Error, FolderPath, ListFilesOp};

impl FolderPath {
    //! List Files

    /// Creates a list-files operation on the folder.
    ///
    /// The files will be returned in lexicographical order.
    pub fn list_files(&self) -> Result<ListFilesOp, Error> {
        if let Some(path) = LocalPath::from(self.path()) {
            return path.list_files().map(|op| ListFilesOp {
                inner: ListFilesOpInner::Local(op),
            });
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::R2Path::from(self.path()) {
            return path.list_files().map(|op| ListFilesOp {
                inner: ListFilesOpInner::R2(op),
            });
        }

        Err(Error::new(
            self.path().clone(),
            ListFiles,
            UnsupportedOperation,
        ))
    }

    /// Creates a list-files operation on the folder where the files may be unsorted.
    ///
    /// The files will be returned in lexicographical order.
    pub fn list_files_unsorted(&self) -> Result<ListFilesOp, Error> {
        if let Some(path) = LocalPath::from(self.path()) {
            return path.list_files_unsorted().map(|op| ListFilesOp {
                inner: ListFilesOpInner::Local(op),
            });
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::R2Path::from(self.path()) {
            return path.list_files().map(|op| ListFilesOp {
                inner: ListFilesOpInner::R2(op),
            });
        }

        Err(Error::new(
            self.path().clone(),
            ListFiles,
            UnsupportedOperation,
        ))
    }
}
