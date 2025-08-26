use std::fs::File;
use std::io::Read;

/// A local file read.
pub struct LocalFileRead {
    pub(crate) file: File,
}

impl Read for LocalFileRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}
