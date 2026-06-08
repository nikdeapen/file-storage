use crate::system::LocalPath;
use crate::Operation;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath, Headers};
use std::io::Write;

impl FilePath {
    //! Write With Headers

    /// Writes the `data` to the file with the given `headers`, overwriting it if it already exists.
    ///
    /// Headers (e.g. `Content-Type`) are applied by backends that support object metadata, such as
    /// Cloudflare R2. The local filesystem backend ignores them.
    #[cfg_attr(not(feature = "r2"), allow(unused_variables))]
    pub fn write_with_headers<D>(&self, data: D, headers: &Headers) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.overwrite(data.as_ref());
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::R2Path::new(self.path()) {
            return path.write_with_headers(data, headers);
        }

        Err(Error::new(self.clone(), Operation::Write, UnknownFileSystem))
    }

    /// Writes the `data` to the file with the given `headers` if the file does not already exist.
    ///
    /// Headers (e.g. `Content-Type`) are applied by backends that support object metadata, such as
    /// Cloudflare R2. The local filesystem backend ignores them.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already exists.
    #[cfg_attr(not(feature = "r2"), allow(unused_variables))]
    pub fn write_with_headers_if_not_exists<D>(
        &self,
        data: D,
        headers: &Headers,
    ) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        if let Some(path) = LocalPath::new(self.path()) {
            return if let Some(mut write) = path.write_if_not_exists()? {
                write
                    .write_all(data.as_ref())
                    .map_err(|e| Error::from_source(self.clone(), Operation::Write, e))?;
                write
                    .close()
                    .map_err(|e| Error::from_source(self.clone(), Operation::Write, e))?;
                Ok(true)
            } else {
                Ok(false)
            };
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::R2Path::new(self.path()) {
            return path.write_with_headers_if_not_exists(data, headers);
        }

        Err(Error::new(self.clone(), Operation::Write, UnknownFileSystem))
    }
}
