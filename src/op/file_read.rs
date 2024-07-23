use std::io::Read;

/// A file read operation.
#[derive(Debug)]
pub struct FileRead {
    pub(crate) inner: FileReadInner,
}

#[derive(Debug)]
pub(crate) enum FileReadInner {
    Local(crate::local::LocalFileRead),
}

impl Read for FileRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            FileReadInner::Local(local) => local.read(buf),
        }
    }
}
