use crate::Operation::Delete;
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::{Error, FilePath, LocalPath, Reason};

impl FilePath {
    //! Delete

    /// Deletes the file.
    ///
    /// # Note
    /// - It succeeds if the file was deleted or did not exist.
    /// - On Cloudflare R2 a missing bucket is an error.
    /// - An empty local folder at the path is deleted. A folder with anything in it is an error.
    /// - A local symlink is deleted itself, never its target.
    pub fn delete(&self) -> Result<(), Error> {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.delete_if_exists().map(|_existed| ());
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.delete();
        }

        Err(Error::new(self.clone(), Delete, Reason::UnknownFileSystem))
    }

    /// Deletes the file if it exists.
    ///
    /// # Atomic
    /// - This is an atomic operation. `Ok(true)` is only returned once per deleted file.
    /// - Cloudflare R2 cannot do this atomically. It returns `UnsupportedOperation`.
    ///
    /// # Note
    /// - It is `true` if the file existed and was deleted. It is `false` if it did not exist.
    /// - An empty local folder at the path is deleted too. It returns `false`. No file was deleted.
    /// - A local folder with anything in it is an error.
    /// - A local symlink is deleted itself, never its target.
    pub fn delete_if_exists(&self) -> Result<bool, Error> {
        if let Some(local) = LocalPath::new(self.path()) {
            return local.delete_if_exists();
        }

        #[cfg(feature = "r2")]
        if R2Path::new(self.path()).is_some() {
            return Err(Error::new(
                self.clone(),
                Delete,
                Reason::UnsupportedOperation,
            ));
        }

        Err(Error::new(self.clone(), Delete, Reason::UnknownFileSystem))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, FilePath, Operation, Reason, StoragePath};

    fn file(path: &str) -> FilePath {
        StoragePath::parse(path).unwrap().to_file().unwrap()
    }

    #[test]
    fn other_hosts_path_is_unknown_file_system() {
        let path: &str = if cfg!(windows) {
            "/folder/file.txt"
        } else {
            "C:\\folder\\file.txt"
        };
        let file: FilePath = file(path);
        let errors: [Error; 2] = [
            file.delete().unwrap_err(),
            file.delete_if_exists().unwrap_err(),
        ];
        for error in errors {
            assert_eq!(error.operation(), Operation::Delete);
            assert_eq!(error.reason(), Reason::UnknownFileSystem);
        }
    }

    #[cfg(feature = "r2")]
    #[test]
    fn r2_delete_if_exists_is_unsupported() {
        let file: FilePath = file("https://account.r2.cloudflarestorage.com/bucket/file.txt");
        let error: Error = file.delete_if_exists().unwrap_err();
        assert_eq!(error.operation(), Operation::Delete);
        assert_eq!(error.reason(), Reason::UnsupportedOperation);
    }
}
