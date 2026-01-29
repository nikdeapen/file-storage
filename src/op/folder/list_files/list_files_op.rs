use crate::system::LocalListFilesOp;
use crate::{Error, FilePath};

/// A list-files operation.
pub struct ListFilesOp {
    pub(crate) inner: ListFilesOpInner,
}

/// A file-system-specific list-files operation.
pub(crate) enum ListFilesOpInner {
    /// A local list-files operation.
    Local(LocalListFilesOp),

    /// A cloudflare r2 list-files operation.
    #[cfg(feature = "r2")]
    R2(crate::R2ListFilesOp),
}

impl Iterator for ListFilesOp {
    type Item = Result<FilePath, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            ListFilesOpInner::Local(op) => op.next(),
            #[cfg(feature = "r2")]
            ListFilesOpInner::R2(op) => op.next(),
        }
    }
}
