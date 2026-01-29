use std::fs::File;
use std::io::Read;

/// A local read operation.
pub struct LocalReadOp {
    pub(crate) file: File,
}

impl Read for LocalReadOp {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buffer)
    }
}
