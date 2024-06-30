use std::io::{Error, Write};

/// A file write operation.
#[derive(Debug)]
pub struct FileWrite {
    pub(crate) inner: FileWriteInner,
}

#[derive(Debug)]
pub(crate) enum FileWriteInner {
    #[cfg(feature = "local")]
    Local(crate::local::LocalFileWrite),
    #[allow(dead_code)]
    Invalid,
}

#[allow(unused_variables)]
impl Write for FileWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            #[cfg(feature = "local")]
            FileWriteInner::Local(local) => local.write(buf),
            FileWriteInner::Invalid => unreachable!(),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.inner {
            #[cfg(feature = "local")]
            FileWriteInner::Local(local) => local.flush(),
            FileWriteInner::Invalid => unreachable!(),
        }
    }
}

impl FileWrite {
    //! Commit

    /// Commits the written data and closes the file.
    pub fn commit(self) -> Result<(), Error> {
        match self.inner {
            #[cfg(feature = "local")]
            FileWriteInner::Local(local) => local.commit(),
            FileWriteInner::Invalid => unreachable!(),
        }
    }
}
