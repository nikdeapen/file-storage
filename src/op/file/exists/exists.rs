use crate::LocalPath;
use crate::Operation::Exists;
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Exists

    /// Checks if the file exists.
    ///
    /// # Note
    /// - A local file system may ignore letter case and Unicode normalization. On macOS and
    ///   Windows `a.txt` matches `A.txt` by default. Cloudflare R2 matches exact bytes.
    /// - On Cloudflare R2 a missing bucket looks like a missing file. It returns `false`.
    /// - A local folder at the path is not a file. It returns `false`.
    /// - Local symlinks are followed. A broken link is not a file.
    /// - Anything else at a local path, such as a FIFO or a device, is an error.
    pub fn exists(&self) -> Result<bool, Error> {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.exists();
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.exists();
        }

        Err(Error::new(self.clone(), Exists, UnknownFileSystem))
    }
}
