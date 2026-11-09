use crate::{Error, FolderPath, StoragePath};
use std::io;
use std::path::PathBuf;

impl FolderPath {
    //! Current Working Directory

    /// Gets the current working directory.
    pub fn current_working_directory() -> Result<Self, io::Error> {
        let path: PathBuf = std::env::current_dir()?;
        match path.into_os_string().into_string() {
            Ok(path) => Ok(StoragePath::parse(path)?.make_folder()),
            Err(os) => Err(Error::path_not_utf8(os.as_ref())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FolderPath, StoragePath};
    use std::path::MAIN_SEPARATOR;

    #[test]
    fn current_working_directory() {
        let mut expected: String = std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        if !expected.ends_with(MAIN_SEPARATOR) {
            expected.push(MAIN_SEPARATOR);
        }

        let cwd: FolderPath = FolderPath::current_working_directory().unwrap();
        let path: &StoragePath = cwd.path();
        assert_eq!(path.path(), expected);
        assert_eq!(path.file_separator(), MAIN_SEPARATOR);
        assert!(path.is_folder());
    }
}
