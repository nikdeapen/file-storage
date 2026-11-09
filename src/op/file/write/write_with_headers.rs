use crate::Operation::Write;
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::Reason::{FileAlreadyExists, UnknownFileSystem};
use crate::{Error, FilePath, Headers, LocalPath};

impl FilePath {
    //! Write With Headers

    /// Writes the `data` to a new file with the `headers`.
    ///
    /// # Note
    /// - It fails with `FileAlreadyExists` if the file already exists.
    /// - The file is created atomically. See `write_with_headers_if_not_exists`.
    /// - Backends with object metadata, such as Cloudflare R2, apply the headers. See `Headers`.
    /// - The local file system ignores the headers.
    pub fn write_with_headers<D>(&self, data: D, headers: &Headers) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        if self.write_with_headers_if_not_exists(data, headers)? {
            Ok(())
        } else {
            Err(Error::new(self.clone(), Write, FileAlreadyExists))
        }
    }

    /// Writes the `data` to a new file with the `headers` if the file does not exist.
    ///
    /// # Atomic
    /// - Only one writer can create the file. The others get `false`.
    ///
    /// # Note
    /// - It is `true` if the file was written. It is `false` if the file already exists.
    /// - Backends with object metadata, such as Cloudflare R2, apply the headers. See `Headers`.
    /// - The local file system ignores the headers.
    /// - A local folder at the path is an error.
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
            return path.write_data_if_not_exists(data.as_ref());
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.write_with_headers_if_not_exists(data, headers);
        }

        Err(Error::new(self.clone(), Write, UnknownFileSystem))
    }
}
