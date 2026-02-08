use file_storage::{FilePath, FolderPath};
use std::error::Error;

/// Test the `list_files` function on the `folder`.
pub fn list_files(folder: &FolderPath) -> Result<(), Box<dyn Error>> {
    folder.delete_files()?;

    let files: &[&str] = &["a", "b/a", "b/b", "b/c", "c"];
    for file in files {
        folder.clone().make_file(file)?.write_empty()?;
    }

    let result: Vec<FilePath> = folder.list_files_as_vec()?;
    let result: Vec<&str> = result
        .iter()
        .map(|file| file.as_str())
        .map(|file| {
            if let Some(s) = file.strip_prefix(folder.as_str()) {
                s
            } else {
                panic!("{}", file);
            }
        })
        .collect();

    assert_eq!(files, result);

    Ok(())
}

/// Test the `delete_files` function on the `folder`.
pub fn delete_files(folder: &FolderPath) -> Result<(), Box<dyn Error>> {
    folder.delete_files()?;

    let files: &[&str] = &["a", "b/a", "b/b", "b/c", "c"];
    for file in files {
        folder.clone().make_file(file)?.write_empty()?;
    }

    assert_eq!(folder.list_files_as_vec()?.len(), files.len());
    folder.delete_files()?;
    assert_eq!(folder.list_files_as_vec()?.len(), 0);

    Ok(())
}
