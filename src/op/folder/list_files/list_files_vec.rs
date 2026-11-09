use crate::{Error, FilePath, FolderPath, ListFilesOp};

impl FolderPath {
    //! List Files as Vec

    /// Lists the files in the folder as a sorted `Vec`.
    ///
    /// # Note
    /// - See `list_files` for which files are listed and how they are sorted.
    pub fn list_files_as_vec(&self) -> Result<Vec<FilePath>, Error> {
        let mut target: Vec<FilePath> = Vec::default();
        self.list_files_to_vec(&mut target)?;
        Ok(target)
    }

    /// Lists the files in the folder as a `Vec` in any order.
    ///
    /// # Note
    /// - See `list_files` for which files are listed.
    pub fn list_files_as_vec_unsorted(&self) -> Result<Vec<FilePath>, Error> {
        let mut target: Vec<FilePath> = Vec::default();
        self.list_files_to_vec_unsorted(&mut target)?;
        Ok(target)
    }

    /// Lists the files in the folder to the end of the `target` `Vec`.
    ///
    /// # Note
    /// - It returns the number of files listed.
    /// - Only the listed files are sorted, not the whole `target`. See `list_files`.
    /// - On error the `target` is left as it was.
    pub fn list_files_to_vec(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        self.append_files(self.list_files()?, target)
    }

    /// Lists the files in the folder to the end of the `target` `Vec` in any order.
    ///
    /// # Note
    /// - It returns the number of files listed. See `list_files`.
    /// - On error the `target` is left as it was.
    pub fn list_files_to_vec_unsorted(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        self.append_files(self.list_files_unsorted()?, target)
    }

    /// Appends the `files` to the `target`.
    ///
    /// # Note
    /// - It returns the number of files appended.
    /// - On error the `target` is cut back to its original length.
    fn append_files(&self, files: ListFilesOp, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        let original_len: usize = target.len();
        for file in files {
            match file {
                Ok(file) => target.push(file),
                Err(error) => {
                    target.truncate(original_len);
                    return Err(error);
                }
            }
        }
        Ok(target.len() - original_len)
    }
}
