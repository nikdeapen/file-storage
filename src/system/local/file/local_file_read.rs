use std::fs::File;
use std::io::Read;

/// A local file read operation.
pub struct LocalFileRead {
    pub(crate) file: File,
}

impl Read for LocalFileRead {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buffer)
    }
}
