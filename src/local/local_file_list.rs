use std::io::Error;

use crate::FilePath;

#[derive(Debug)]
pub struct LocalFileList {
    pub files_reversed: Vec<FilePath>,
}

impl Iterator for LocalFileList {
    type Item = Result<FilePath, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(file) = self.files_reversed.pop() {
            Some(Ok(file))
        } else {
            None
        }
    }
}
