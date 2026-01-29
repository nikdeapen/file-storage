use crate::{Error, FilePath, FolderPath, ListFilesOp};

impl FolderPath {
    //! List Files as Vec

    /// Lists the files in the folder as a `Vec<FilePath>` sorted in lexicographical order.
    ///
    /// Returns `Ok(sorted_files)`.
    pub fn list_files_as_vec(&self) -> Result<Vec<FilePath>, Error> {
        let mut vec: Vec<FilePath> = Vec::default();
        self.list_files_to_vec(&mut vec)?;
        Ok(vec)
    }

    /// Lists the files as a `Vec<FilePath>`.
    ///
    /// Returns `Ok(files)`.
    pub fn list_files_as_vec_unsorted(&self) -> Result<Vec<FilePath>, Error> {
        let mut target: Vec<FilePath> = Vec::new();
        self.list_files_to_vec_unsorted(&mut target)?;
        Ok(target)
    }

    /// Lists the files from the folder and appends them to the `target` `Vec`.
    ///
    /// Returns `Ok(file_count)`.
    ///
    /// # Note
    /// Only the appended files will be sorted, not the entire `target`.
    pub fn list_files_to_vec(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        let mut count: usize = 0;
        let files: ListFilesOp = self.list_files()?;
        for file in files {
            target.push(file?);
            count += 1;
        }
        Ok(count)
    }

    /// Lists the files to the `target` `Vec`.
    ///
    /// Returns `Ok(file_count)`.
    pub fn list_files_to_vec_unsorted(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        let mut count: usize = 0;
        let files: ListFilesOp = self.list_files_unsorted()?;
        for file in files {
            target.push(file?);
            count += 1;
        }
        Ok(count)
    }
}
