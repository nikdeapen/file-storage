use std::fs::File;
use std::io::{Error, Write};

#[derive(Debug)]
pub struct LocalFileWrite {
    pub(crate) file: File,
}

impl Write for LocalFileWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl LocalFileWrite {
    pub fn commit(self) -> Result<(), Error> {
        self.file.sync_all()
    }
}
