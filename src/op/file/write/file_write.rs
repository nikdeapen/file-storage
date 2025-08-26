use crate::system::LocalFileWrite;
use std::io::Write;

/// A file write operation.
pub struct FileWrite {
    pub(crate) inner: FileWriteInner,
}

pub(crate) enum FileWriteInner {
    Local(LocalFileWrite),
}

impl Write for FileWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            FileWriteInner::Local(local) => local.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.inner {
            FileWriteInner::Local(local) => local.flush(),
        }
    }
}

impl FileWrite {
    //! Close

    /// Closes the file.
    pub fn close(self) -> Result<(), std::io::Error> {
        match self.inner {
            FileWriteInner::Local(local) => local.close(),
        }
    }
}
