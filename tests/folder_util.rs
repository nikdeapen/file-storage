use file_storage::{FilePath, FolderPath};
use std::error::Error;

/// The files each folder test writes, in sorted order.
///
/// # Note
/// - `b.txt` sorts before `b/a` since `.` comes before `/`.
const FILES: &[&str] = &["a", "b.txt", "b/a", "b/b", "b/c", "c"];

/// Writes the test files to the `folder`.
fn write_files(folder: &FolderPath) -> Result<(), Box<dyn Error>> {
    for file in FILES {
        folder.clone_append(file).to_file()?.write_empty()?;
    }
    Ok(())
}

/// Gets the paths of the `files` relative to the `folder`.
fn names<'a>(folder: &FolderPath, files: &'a [FilePath]) -> Vec<&'a str> {
    files
        .iter()
        .map(|file| match file.as_str().strip_prefix(folder.as_str()) {
            Some(name) => name,
            None => panic!("{} is not in {}", file, folder),
        })
        .collect()
}

/// Tests the `list_files` function on the `folder`.
pub fn list_files(folder: &FolderPath) -> Result<(), Box<dyn Error>> {
    folder.delete_files()?;
    assert!(folder.list_files_as_vec()?.is_empty());

    write_files(folder)?;
    let files: Vec<FilePath> = folder.list_files_as_vec()?;
    assert_eq!(names(folder, &files), FILES);

    // A subfolder lists only the files beneath it. `b.txt` shares its prefix but is not in `b/`.
    let sub: FolderPath = folder.clone_append("b/").to_folder()?;
    let sub_files: Vec<FilePath> = sub.list_files_as_vec()?;
    assert_eq!(names(folder, &sub_files), ["b/a", "b/b", "b/c"]);

    // A missing folder lists nothing.
    let missing: FolderPath = folder.clone_append("missing/").to_folder()?;
    assert!(missing.list_files_as_vec()?.is_empty());

    // Listing to a `Vec` appends and sorts only the listed files.
    let marker: FilePath = folder.clone_append("z").to_file()?;
    let mut target: Vec<FilePath> = vec![marker.clone()];
    assert_eq!(folder.list_files_to_vec(&mut target)?, FILES.len());
    assert_eq!(target[0], marker);
    assert_eq!(target[1..], files[..]);

    // Unsorted lists the same files.
    let mut unsorted: Vec<FilePath> = folder.list_files_as_vec_unsorted()?;
    unsorted.sort();
    assert_eq!(unsorted, files);

    Ok(())
}

/// Tests the `delete_files` function on the `folder`.
pub fn delete_files(folder: &FolderPath) -> Result<(), Box<dyn Error>> {
    folder.delete_files()?;

    // A missing folder has nothing to delete.
    folder
        .clone_append("missing/")
        .to_folder()?
        .delete_files()?;

    write_files(folder)?;
    assert_eq!(folder.list_files_as_vec()?.len(), FILES.len());

    // A subfolder deletes only the files beneath it. `b.txt` shares its prefix but is not in `b/`.
    folder.clone_append("b/").to_folder()?.delete_files()?;
    let files: Vec<FilePath> = folder.list_files_as_vec()?;
    assert_eq!(names(folder, &files), ["a", "b.txt", "c"]);

    folder.delete_files()?;
    assert!(folder.list_files_as_vec()?.is_empty());

    Ok(())
}
