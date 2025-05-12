use crate::system::LocalReadFile;
use std::io::Read;

/// A file read operation.
pub struct ReadFile {
    pub(crate) inner: ReadFileInner,
}

pub(crate) enum ReadFileInner {
    Local(LocalReadFile),
}

impl Read for ReadFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            ReadFileInner::Local(local) => local.read(buf),
        }
    }
}
