use crate::system::local::is_not_found;
use crate::system::{LocalPath, LocalReadOp};
use crate::{Error, Operation};
use std::fs::File;
use std::io::Read;

impl<'a> LocalPath<'a> {
    //! Read

    /// See `FilePath::read_if_exists`.
    pub(crate) fn read_if_exists(self) -> Result<Option<LocalReadOp>, Error> {
        Ok(self.open()?.map(|file| LocalReadOp { file }))
    }

    /// See `FilePath::read_to_vec_if_exists`.
    ///
    /// # Note
    /// - `File::read_to_end` reserves the file's size up front.
    pub(crate) fn read_to_vec_if_exists(
        self,
        target: &mut Vec<u8>,
    ) -> Result<Option<usize>, Error> {
        if let Some(mut file) = self.open()? {
            let read: usize = file
                .read_to_end(target)
                .map_err(|e| Error::from_source(self.path.clone(), Operation::Read, e))?;
            Ok(Some(read))
        } else {
            Ok(None)
        }
    }

    /// Opens the file for reading.
    ///
    /// # Note
    /// - It is `None` if the file does not exist.
    /// - The path is checked like in `exists` before it is opened. A folder reads as no file.
    ///   Anything else that is not a file is an error.
    /// - Checking first keeps the open from blocking on a FIFO. Windows also cannot open a folder
    ///   as a file.
    fn open(self) -> Result<Option<File>, Error> {
        if !self.has_file(Operation::Read)? {
            return Ok(None);
        }
        match File::open(self.std_path()) {
            Ok(file) => Ok(Some(file)),
            Err(error) if is_not_found(&error) => Ok(None),
            Err(error) => Err(Error::from_source(
                self.path.clone(),
                Operation::Read,
                error,
            )),
        }
    }
}
