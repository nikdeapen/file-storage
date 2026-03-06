use crate::system::LocalPath;
use crate::Error;
use crate::Operation::Delete;
use std::io::ErrorKind::NotFound;
use std::path::Path;

impl<'a> LocalPath<'a> {
    //! Delete Files

    /// Deletes all files in the folder recursively.
    pub fn delete_files(&self) -> Result<(), Error> {
        self.delete_files_in_dir(Path::new(self.path.as_str()))
    }

    fn delete_files_in_dir(&self, dir: &Path) -> Result<(), Error> {
        let read_dir = match std::fs::read_dir(dir) {
            Ok(read_dir) => read_dir,
            Err(e) if e.kind() == NotFound => return Ok(()),
            Err(e) => return Err(Error::from_source(self.path.clone(), Delete, e)),
        };
        for entry in read_dir {
            let entry = entry.map_err(|e| Error::from_source(self.path.clone(), Delete, e))?;
            let file_type = entry
                .file_type()
                .map_err(|e| Error::from_source(self.path.clone(), Delete, e))?;
            if file_type.is_file() {
                std::fs::remove_file(entry.path())
                    .map_err(|e| Error::from_source(self.path.clone(), Delete, e))?;
            } else if file_type.is_dir() {
                self.delete_files_in_dir(&entry.path())?;
            }
        }
        Ok(())
    }
}
