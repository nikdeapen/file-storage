use crate::FilePath;

impl FilePath {
    //! File Name

    /// Gets the file name.
    pub fn file_name(&self) -> &str {
        let extension: &str = self.path().extension();
        let fs: char = self.path().file_separator();
        if let Some(lfs_index) = extension.rfind(|c| c == fs) {
            &extension[(lfs_index + fs.len_utf8())..]
        } else {
            extension
        }
    }

    /// Gets the file name split from the extension.
    pub fn file_name_and_extension(&self) -> (&str, &str) {
        let file_name: &str = self.file_name();
        if let Some(dot) = file_name.as_bytes().iter().position(|c| *c == b'.') {
            file_name.split_at(dot)
        } else {
            (file_name, "")
        }
    }
}
