use std::fs::File;
use std::io::Read;

/// A local read operation.
pub struct LocalReadFile {
    pub(crate) file: File,
}

impl Read for LocalReadFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}
