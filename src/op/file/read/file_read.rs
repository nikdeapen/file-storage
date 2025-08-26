use std::io::Read;

/// A file read operation.
pub struct FileRead {
    pub(crate) inner: ReadFileInner,
}

pub(crate) enum ReadFileInner {
    Local(crate::LocalFileRead),
    #[cfg(feature = "r2")]
    R2(crate::R2FileRead),
}

impl Read for FileRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            ReadFileInner::Local(local) => local.read(buf),
            #[cfg(feature = "r2")]
            ReadFileInner::R2(r2) => r2.read(buf),
        }
    }
}
