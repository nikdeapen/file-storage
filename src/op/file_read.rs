use std::io::Read;

/// A file read operation.
#[derive(Debug)]
pub struct FileRead {
    pub(crate) inner: FileReadInner,
}

#[derive(Debug)]
pub(crate) enum FileReadInner {
    #[cfg(feature = "local")]
    Local(crate::local::LocalFileRead),
    #[allow(dead_code)]
    Invalid,
}

#[allow(unused_variables)]
impl Read for FileRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            #[cfg(feature = "local")]
            FileReadInner::Local(local) => local.read(buf),
            FileReadInner::Invalid => unreachable!(),
        }
    }
}
