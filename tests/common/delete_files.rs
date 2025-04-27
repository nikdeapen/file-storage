use file_storage::{FilePath, FolderPath};
use std::error::Error;

#[allow(dead_code)]
pub fn test_delete_files(folder: &FolderPath) -> Result<(), Box<dyn Error>> {
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

    folder.delete_files()?;
    let files: Vec<FilePath> = folder.list_files_as_vec()?;
    assert_eq!(files.len(), 0);

    Ok(())
}
