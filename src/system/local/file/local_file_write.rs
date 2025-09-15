use std::fs::File;
use std::io::Write;

/// A local file write operation.
pub struct LocalFileWrite {
    pub(crate) file: File,
}

impl Write for LocalFileWrite {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.file.write(buffer)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl LocalFileWrite {
    //! Close

    /// Closes the file.
    pub fn close(self) -> Result<(), std::io::Error> {
        self.file.sync_all()
    }
}
