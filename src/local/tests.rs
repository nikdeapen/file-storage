use crate::{FilePath, FolderPath};

#[test]
fn read_write_exists_delete() -> Result<(), Box<dyn std::error::Error>> {
    let file: FilePath = FilePath::temp()?;
    assert!(!file.exists()?);

    assert!(file.write_str_if_not_exists("Hello, World!")?);
    assert!(file.exists()?);

    assert!(!file.write_data_if_not_exists("Goodbye, World!")?);
    assert!(file.exists()?);

    assert!(file.delete_if_exists()?);
    assert!(!file.exists()?);

    assert!(!file.delete_if_exists()?);

    Ok(())
}

#[test]
#[cfg(feature = "tempfile")]
fn list() -> Result<(), Box<dyn std::error::Error>> {
    let folder: FolderPath = FolderPath::temp()?;

    let aa: FilePath = folder.clone().make_file("a/a").unwrap();
    let ab: FilePath = folder.clone().make_file("a/b").unwrap();
    let ac: FilePath = folder.clone().make_file("a/c").unwrap();
    let ba: FilePath = folder.clone().make_file("b/a").unwrap();
    let bb: FilePath = folder.clone().make_file("b/b").unwrap();
    let bc: FilePath = folder.clone().make_file("b/c").unwrap();

    aa.write_empty_file()?;
    ab.write_empty_file()?;
    ac.write_empty_file()?;
    ba.write_empty_file()?;
    bb.write_empty_file()?;
    bc.write_empty_file()?;

    let files: Vec<FilePath> = folder.list_files()?.map(|r| r.unwrap()).collect();
    assert_eq!(files.len(), 6);
    assert!(files[0].path().path().ends_with("/a/a"));
    assert!(files[1].path().path().ends_with("/a/b"));
    assert!(files[2].path().path().ends_with("/a/c"));
    assert!(files[3].path().path().ends_with("/b/a"));
    assert!(files[4].path().path().ends_with("/b/b"));
    assert!(files[5].path().path().ends_with("/b/c"));

    Ok(())
}
