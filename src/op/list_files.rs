use std::io::Error;

use crate::FilePath;

/// A file list operation.
#[derive(Debug)]
pub struct ListFiles {
    pub(crate) inner: FileListInner,
}

#[derive(Debug)]
pub(crate) enum FileListInner {
    Local(crate::local::LocalFileList),
}

#[allow(unused_variables)]
impl Iterator for ListFiles {
    type Item = Result<FilePath, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            FileListInner::Local(local) => local.next(),
        }
    }
}
