use crate::system::LocalPath;
use crate::Error;
use crate::Operation::DeleteFiles;
use std::io::ErrorKind::NotFound;
use std::path::Path;

impl<'a> LocalPath<'a> {
    //! Delete Files

    /// Deletes all files in the folder recursively.
    pub fn delete_files(self) -> Result<(), Error> {
        self.delete_files_in_dir(Path::new(self.path.as_str()))
    }

    fn delete_files_in_dir(self, dir: &Path) -> Result<(), Error> {
        let read_dir: std::fs::ReadDir = match std::fs::read_dir(dir) {
            Ok(read_dir) => read_dir,
            Err(e) if e.kind() == NotFound => return Ok(()),
            Err(e) => return Err(Error::from_source(self.path.clone(), DeleteFiles, e)),
        };
        for entry in read_dir {
            let entry: std::fs::DirEntry = entry.map_err(|e| Error::from_source(self.path.clone(), DeleteFiles, e))?;
            let file_type: std::fs::FileType = entry
                .file_type()
                .map_err(|e| Error::from_source(self.path.clone(), DeleteFiles, e))?;
            if file_type.is_file() {
                std::fs::remove_file(entry.path())
                    .map_err(|e| Error::from_source(self.path.clone(), DeleteFiles, e))?;
            } else if file_type.is_dir() {
                self.delete_files_in_dir(&entry.path())?;
            }
        }
        Ok(())
    }
}
