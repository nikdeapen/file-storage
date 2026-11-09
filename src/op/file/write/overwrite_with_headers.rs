use crate::Operation::Write;
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath, Headers, LocalPath};

impl FilePath {
    //! Overwrite With Headers

    /// Writes the `data` to the file with the `headers`, replacing the file if it exists.
    ///
    /// # Note
    /// - Backends with object metadata, such as Cloudflare R2, apply the headers. See `Headers`.
    /// - The local file system ignores the headers.
    /// - Readers see the old file or the new one, never part of one.
    /// - A local symlink at the path is followed. Its target is replaced.
    /// - A local overwrite makes a new file. It has default permissions. Hard links to the old
    ///   file keep the old data.
    /// - A local folder at the path is an error.
    #[cfg_attr(not(feature = "r2"), allow(unused_variables))]
    pub fn overwrite_with_headers<D>(&self, data: D, headers: &Headers) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.overwrite(data.as_ref());
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.overwrite_with_headers(data, headers);
        }

        Err(Error::new(self.clone(), Write, UnknownFileSystem))
    }
}
