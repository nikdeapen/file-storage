use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

/// A local write operation.
///
/// # Temp File
/// - The data goes to a temp file in the target's folder. `close` moves it to the target.
/// - A create-only write links the temp file to the target. Linking fails if the target exists.
/// - A replacing write renames the temp file over the target.
/// - Dropping the operation without calling `close` deletes the temp file. A crash can leave it.
pub(crate) struct LocalWriteOp {
    file: File,
    temp: PathBuf,
    target: PathBuf,

    /// Whether `close` replaces the target. Otherwise it only creates the target.
    replace: bool,

    /// The folders that gain a new entry for this file, from its parent up.
    folders: Vec<PathBuf>,

    /// Whether `close` moved the temp file to the target.
    published: bool,
}

impl LocalWriteOp {
    //! Construction

    /// Creates the [LocalWriteOp].
    pub(in crate::system::local) fn new(
        file: File,
        temp: PathBuf,
        target: PathBuf,
        replace: bool,
        folders: Vec<PathBuf>,
    ) -> Self {
        Self {
            file,
            temp,
            target,
            replace,
            folders,
            published: false,
        }
    }
}

impl LocalWriteOp {
    //! Close

    /// Closes the file and moves it to the target.
    ///
    /// # Note
    /// - A create-only write fails with `AlreadyExists` if the target was created first.
    /// - The file is synced before it is moved. On Unix the `folders` are synced last. A new name
    ///   only survives a crash once its folder is synced.
    /// - Windows cannot open a folder as a file. Its folders are not synced.
    /// - If syncing a folder fails, the file is already in place. The error is still returned.
    pub(crate) fn close(mut self) -> Result<(), io::Error> {
        self.file.sync_all()?;
        self.publish()?;
        self.published = true;
        if cfg!(unix) {
            for folder in &self.folders {
                File::open(folder)?.sync_all()?;
            }
        }
        Ok(())
    }

    /// Moves the temp file to the target.
    ///
    /// # Note
    /// - A link leaves the temp file in place, so it is deleted after. If that fails it is left
    ///   behind. The write has already succeeded.
    fn publish(&self) -> io::Result<()> {
        if self.replace {
            return std::fs::rename(&self.temp, &self.target);
        }
        match std::fs::hard_link(&self.temp, &self.target) {
            Ok(()) => {
                let _ignored = std::fs::remove_file(&self.temp);
                Ok(())
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "the file already exists",
            )),
            Err(error) => Err(error),
        }
    }
}

impl Write for LocalWriteOp {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.file.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

impl Drop for LocalWriteOp {
    /// Deletes the temp file unless `close` moved it to the target.
    fn drop(&mut self) {
        if !self.published {
            let _ignored = std::fs::remove_file(&self.temp);
        }
    }
}
