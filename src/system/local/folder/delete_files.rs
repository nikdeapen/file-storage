use crate::system::local::is_not_found;
use crate::system::LocalPath;
use crate::Error;
use crate::Operation::DeleteFiles;
use std::fs::{DirEntry, FileType, ReadDir};
use std::path::{Path, PathBuf};

impl<'a> LocalPath<'a> {
    //! Delete Files

    /// Deletes all files in the folder recursively.
    ///
    /// # Note
    /// - Files that disappear during the walk are skipped.
    /// - Subfolders it empties are deleted too. The folder itself is kept.
    /// - Symlinks are deleted themselves. Their targets are never touched.
    pub(crate) fn delete_files(self) -> Result<(), Error> {
        self.delete_files_in_dir(self.std_path())?;
        Ok(())
    }

    /// Deletes the files in the `dir` and its subfolders.
    ///
    /// # Note
    /// - It is `true` if the `dir` was left empty.
    /// - Subfolders it empties are deleted too.
    fn delete_files_in_dir(self, dir: &Path) -> Result<bool, Error> {
        let read_dir: ReadDir = match std::fs::read_dir(dir) {
            Ok(read_dir) => read_dir,
            Err(e) if is_not_found(&e) => return Ok(true),
            Err(e) => return Err(self.delete_files_error(e)),
        };
        let mut emptied: bool = true;
        for entry in read_dir {
            let entry: DirEntry = entry.map_err(|e| self.delete_files_error(e))?;
            let file_type: FileType = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(e) if is_not_found(&e) => continue,
                Err(e) => return Err(self.delete_files_error(e)),
            };
            if file_type.is_file() {
                match std::fs::remove_file(entry.path()) {
                    Err(e) if !is_not_found(&e) => return Err(self.delete_files_error(e)),
                    _ => {}
                }
            } else if file_type.is_symlink() {
                self.remove_link(&entry.path(), file_type)?;
            } else if file_type.is_dir() {
                let subfolder: PathBuf = entry.path();
                let removed: bool =
                    self.delete_files_in_dir(&subfolder)? && self.remove_emptied_dir(&subfolder)?;
                emptied &= removed;
            } else {
                // Other entries, such as sockets, are not files. They are kept.
                emptied = false;
            }
        }
        Ok(emptied)
    }

    /// Removes the `dir` after its files were deleted.
    ///
    /// # Note
    /// - It is `true` if the `dir` is gone.
    /// - A file created in the `dir` during the walk keeps it. That is not an error.
    fn remove_emptied_dir(self, dir: &Path) -> Result<bool, Error> {
        match std::fs::remove_dir(dir) {
            Ok(()) => Ok(true),
            Err(e) if is_not_found(&e) => Ok(true),
            Err(e) => {
                let refilled: bool = std::fs::read_dir(dir)
                    .map(|mut entries| entries.next().is_some())
                    .unwrap_or(false);
                if refilled {
                    Ok(false)
                } else {
                    Err(self.delete_files_error(e))
                }
            }
        }
    }

    /// Removes the symlink at the `link`, never its target.
    ///
    /// # Note
    /// - Windows removes a link to a folder with `remove_dir`. See `is_folder_link`.
    fn remove_link(self, link: &Path, file_type: FileType) -> Result<(), Error> {
        let result: std::io::Result<()> = if self.is_folder_link(file_type) {
            std::fs::remove_dir(link)
        } else {
            std::fs::remove_file(link)
        };
        match result {
            Err(e) if !is_not_found(&e) => Err(self.delete_files_error(e)),
            _ => Ok(()),
        }
    }

    /// Creates a delete-files error from the `source`.
    fn delete_files_error(self, source: std::io::Error) -> Error {
        Error::from_source(self.path.clone(), DeleteFiles, source)
    }
}
