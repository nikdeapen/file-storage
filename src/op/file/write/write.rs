use crate::op::WriteOpInner;
use crate::Operation::Write;
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::Reason::{FileAlreadyExists, UnknownFileSystem};
use crate::{Error, FilePath, LocalPath, WriteOp};

impl FilePath {
    //! Write

    /// Opens a write operation to create the file.
    ///
    /// # Note
    /// - It fails with `FileAlreadyExists` if the file already exists.
    /// - The file is created atomically. See `write_if_not_exists`.
    /// - The operation must be closed with `WriteOp::close` to commit the write.
    pub fn write(&self) -> Result<WriteOp, Error> {
        if let Some(write) = self.write_if_not_exists()? {
            Ok(write)
        } else {
            Err(Error::new(self.clone(), Write, FileAlreadyExists))
        }
    }

    /// Opens a write operation to create the file if it does not exist.
    ///
    /// # Atomic
    /// - Only one writer can create the file.
    /// - The file is created by `WriteOp::close`. If another writer created it first, `close`
    ///   fails with `FileAlreadyExists`.
    ///
    /// # Note
    /// - It is `None` if the file already exists.
    /// - The operation must be closed with `WriteOp::close` to commit the write.
    /// - A local write goes to a temp file in the same folder until then. See `WriteOp`.
    /// - A local symlink at the path is followed. Writing through a broken link creates its target.
    /// - A local folder at the path is an error.
    pub fn write_if_not_exists(&self) -> Result<Option<WriteOp>, Error> {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.write_if_not_exists().map(|op| {
                op.map(|op| WriteOp {
                    file: self.clone(),
                    inner: WriteOpInner::Local(op),
                })
            });
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.write_if_not_exists().map(|op| {
                op.map(|op| WriteOp {
                    file: self.clone(),
                    inner: WriteOpInner::R2(op),
                })
            });
        }

        Err(Error::new(self.clone(), Write, UnknownFileSystem))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, FilePath, Headers, Operation, Reason, StoragePath};

    #[test]
    fn other_hosts_path_is_unknown_file_system() {
        let path: &str = if cfg!(windows) {
            "/folder/file.txt"
        } else {
            "C:\\folder\\file.txt"
        };
        let file: FilePath = StoragePath::parse(path).unwrap().to_file().unwrap();
        let errors: [Error; 3] = [
            file.write().err().unwrap(),
            file.write_data("data").unwrap_err(),
            file.overwrite_with_headers("data", &Headers::new())
                .unwrap_err(),
        ];
        for error in errors {
            assert_eq!(error.operation(), Operation::Write);
            assert_eq!(error.reason(), Reason::UnknownFileSystem);
        }
    }
}
