use std::fs::File;
use std::io::Write;

/// A local write operation.
pub struct LocalWriteOp {
    pub(crate) file: File,
}

impl Write for LocalWriteOp {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.file.write(buffer)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl LocalWriteOp {
    //! Close

    /// Closes the file.
    pub fn close(self) -> Result<(), std::io::Error> {
        self.file.sync_all()
    }
}
