use file_storage::{FilePath, FolderPath};
use std::error::Error;

#[allow(dead_code)]
pub fn test_list_files(folder: &FolderPath) -> Result<(), Box<dyn Error>> {
    folder.delete_files()?;

    assert_eq!(folder.list_files_as_vec()?, vec![]);

    let endings: &[&str] = &["a", "b/a", "b/b", "b/c", "c", "d/a", "d/b", "d/c", "e"];
    for ending in endings {
        folder
            .clone_append(ending)
            .to_file()
            .map_err(|e| e.to_string())?
            .write_empty()?;
    }

    let files: Vec<FilePath> = folder.list_files_as_vec()?;
    assert_eq!(files.len(), endings.len());
    for (i, ending) in endings.iter().enumerate() {
        let path: &str = files[i].path().as_str();
        assert_eq!(path.len(), folder.len() + ending.len());
        assert_eq!(&path[..folder.len()], folder.as_str());
        assert_eq!(&path[folder.len()..], *ending);
    }

    Ok(())
}
