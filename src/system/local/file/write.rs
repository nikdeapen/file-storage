use crate::system::local::is_not_found;
use crate::system::{LocalPath, LocalWriteOp};
use crate::{Error, Operation};
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// The count of temp files this process has created. It keeps their names apart.
static TEMP_FILES: AtomicU64 = AtomicU64::new(0);

impl<'a> LocalPath<'a> {
    //! Write

    /// See `FilePath::write_if_not_exists`.
    ///
    /// # Note
    /// - The check here only finds an existing file early. The atomic step is the link in `close`.
    pub(crate) fn write_if_not_exists(self) -> Result<Option<LocalWriteOp>, Error> {
        let target: PathBuf = self.follow_symlinks()?;
        match std::fs::symlink_metadata(&target) {
            Ok(metadata) if metadata.is_dir() => Err(self.folder_error()),
            Ok(_metadata) => Ok(None),
            Err(error) if is_not_found(&error) => self.open_temp(target, false).map(Some),
            Err(error) => Err(self.write_error(error)),
        }
    }

    /// See `FilePath::write_data_if_not_exists`.
    pub(crate) fn write_data_if_not_exists(self, data: &[u8]) -> Result<bool, Error> {
        let Some(mut op) = self.write_if_not_exists()? else {
            return Ok(false);
        };
        op.write_all(data).map_err(|e| self.write_error(e))?;
        match op.close() {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == AlreadyExists => Ok(false),
            Err(error) => Err(self.write_error(error)),
        }
    }

    /// See `FilePath::overwrite_with_headers`.
    ///
    /// # Note
    /// - A symlink at the path is followed. Its target is replaced.
    pub(crate) fn overwrite(self, data: &[u8]) -> Result<(), Error> {
        let target: PathBuf = self.follow_symlinks()?;
        if std::fs::symlink_metadata(&target).is_ok_and(|metadata| metadata.is_dir()) {
            return Err(self.folder_error());
        }
        let mut op: LocalWriteOp = self.open_temp(target, true)?;
        op.write_all(data).map_err(|e| self.write_error(e))?;
        op.close().map_err(|e| self.write_error(e))
    }

    /// Opens a write to a temp file that `close` moves to the `target`.
    ///
    /// # Note
    /// - The temp file is in the target's folder. Links and renames only work within one file
    ///   system.
    fn open_temp(self, target: PathBuf, replace: bool) -> Result<LocalWriteOp, Error> {
        let folders: Vec<PathBuf> = self.prepare(&target)?;
        let Some(folder) = target.parent() else {
            let message: &str = "the file path has no folder";
            return Err(Error::from_message(
                self.path.clone(),
                Operation::Write,
                message,
            ));
        };
        let (file, temp) = self.create_temp(folder)?;
        Ok(LocalWriteOp::new(file, temp, target, replace, folders))
    }

    /// Creates a new temp file in the `folder`.
    ///
    /// # Note
    /// - The name holds the process id and a count. A name left by a crash is skipped.
    fn create_temp(self, folder: &Path) -> Result<(File, PathBuf), Error> {
        const MAX_ATTEMPTS: usize = 100;
        for _ in 0..MAX_ATTEMPTS {
            let count: u64 = TEMP_FILES.fetch_add(1, Ordering::Relaxed);
            let name: String = format!(".file-storage-{}-{}.tmp", std::process::id(), count);
            let temp: PathBuf = folder.join(name);
            match File::create_new(&temp) {
                Ok(file) => return Ok((file, temp)),
                Err(error) if error.kind() == AlreadyExists => continue,
                Err(error) => return Err(self.write_error(error)),
            }
        }
        let message: &str = "could not find a free name for a temp file";
        Err(Error::from_message(
            self.path.clone(),
            Operation::Write,
            message,
        ))
    }

    /// Gets the path a write goes to, following symlinks at the file path.
    ///
    /// # Note
    /// - Linking or renaming onto a symlink would replace the link itself. Following it writes to
    ///   the link's target, like reads do.
    fn follow_symlinks(self) -> Result<PathBuf, Error> {
        const MAX_SYMLINKS: usize = 40;
        let mut file: PathBuf = self.std_path().to_path_buf();
        for _ in 0..MAX_SYMLINKS {
            let is_symlink: bool = std::fs::symlink_metadata(&file)
                .map(|metadata| metadata.file_type().is_symlink())
                .unwrap_or(false);
            if !is_symlink {
                return Ok(file);
            }
            let target: PathBuf = std::fs::read_link(&file).map_err(|e| self.write_error(e))?;
            file = match file.parent() {
                Some(parent) => parent.join(target),
                None => target,
            };
        }
        let message: &str = "too many symlinks at the file path";
        Err(Error::from_message(
            self.path.clone(),
            Operation::Write,
            message,
        ))
    }

    /// Creates the missing parent folders of the `file`.
    ///
    /// # Note
    /// - It returns the folders that gain a new entry, from the file's parent up.
    /// - A new entry only survives a crash once its folder is synced.
    fn prepare(self, file: &Path) -> Result<Vec<PathBuf>, Error> {
        let Some(parent) = file.parent() else {
            return Ok(Vec::default());
        };
        let mut existing: &Path = parent;
        while !existing.exists() {
            match existing.parent() {
                Some(next) => existing = next,
                None => break,
            }
        }
        std::fs::create_dir_all(parent).map_err(|e| self.write_error(e))?;
        let created: Vec<PathBuf> = parent
            .ancestors()
            .take_while(|folder| *folder != existing)
            .map(Path::to_path_buf)
            .collect();
        Ok(created
            .into_iter()
            .chain(std::iter::once(existing.to_path_buf()))
            .collect())
    }

    /// Creates a write error from the `source`.
    fn write_error(self, source: std::io::Error) -> Error {
        Error::from_source(self.path.clone(), Operation::Write, source)
    }

    /// Creates the error for a folder at the file path.
    fn folder_error(self) -> Error {
        let message: &str = "a folder is at the file path";
        Error::from_message(self.path.clone(), Operation::Write, message)
    }
}

#[cfg(test)]
mod tests {
    use crate::{LocalPath, StoragePath};
    use std::path::PathBuf;

    #[test]
    fn prepare_returns_the_folders_that_gain_an_entry() {
        let temp: PathBuf = std::env::temp_dir();
        let root: PathBuf = temp.join(format!("file-storage-prepare-{}", std::process::id()));
        let file: PathBuf = root.join("x").join("file.txt");
        let path: StoragePath = StoragePath::parse(file.to_str().unwrap()).unwrap();
        let local: LocalPath = LocalPath::new(&path).unwrap();

        let folders: Vec<PathBuf> = local.prepare(&file).unwrap();
        assert_eq!(folders, [root.join("x"), root.clone(), temp]);

        let folders: Vec<PathBuf> = local.prepare(&file).unwrap();
        assert_eq!(folders, [root.join("x")]);

        std::fs::remove_dir_all(&root).unwrap();
    }
}
