use std::io::Error;

use crate::FilePath;

/// A file list operation.
#[derive(Debug)]
pub struct FileList {
    pub(crate) inner: FileListInner,
}

#[derive(Debug)]
pub(crate) enum FileListInner {
    #[cfg(feature = "local")]
    Local(crate::local::LocalFileList),
    #[allow(dead_code)]
    Invalid,
}

#[allow(unused_variables)]
impl Iterator for FileList {
    type Item = Result<FilePath, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            #[cfg(feature = "local")]
            FileListInner::Local(local) => local.next(),
            FileListInner::Invalid => unreachable!(),
        }
    }
}
