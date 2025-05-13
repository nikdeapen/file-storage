use crate::system::{LocalReadFile, R2ReadFile};
use std::io::Read;

/// A file read operation.
pub struct FileRead {
    pub(crate) inner: ReadFileInner,
}

pub(crate) enum ReadFileInner {
    Local(LocalReadFile),
    R2(R2ReadFile),
}

impl Read for FileRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            ReadFileInner::Local(local) => local.read(buf),
            ReadFileInner::R2(r2) => r2.read(buf),
        }
    }
}
