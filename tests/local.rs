#[allow(dead_code)]
mod file_util;
#[allow(dead_code)]
mod folder_util;

mod tests {
    use crate::file_util;
    use crate::folder_util;
    use file_storage::{FilePath, FolderPath, Headers, ListFilesOp, Reason, StoragePath};
    use std::error::Error;
    use std::path::Path;

    #[test]
    fn file() -> Result<(), Box<dyn Error>> {
        let file: FilePath = FilePath::temp()?;
        file_util::test_file(&file, true)?;
        Ok(())
    }

    #[test]
    fn folder() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;

        folder_util::list_files(&folder)?;
        folder_util::delete_files(&folder)?;

        Ok(())
    }

    #[test]
    fn read_to_string_keeps_target_on_bad_utf8() -> Result<(), Box<dyn Error>> {
        let file: FilePath = FilePath::temp()?;
        file.write_data([0xFFu8, 0xFE])?;

        let mut target: String = String::from("kept");
        let reason: Reason = file.read_to_string(&mut target).unwrap_err().reason();
        assert_eq!(reason, Reason::FileContentNotUtf8);
        assert_eq!(target, "kept");

        file.delete()?;
        Ok(())
    }

    #[test]
    fn list_files_skips_a_deleted_file() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;
        for name in ["a", "b", "c"] {
            folder.clone_append(name).to_file()?.write_empty()?;
        }

        let mut files: ListFilesOp = folder.list_files()?;
        let first: FilePath = files.next().ok_or("no first file")??;
        folder.clone_append("b").to_file()?.delete()?;
        let rest: Vec<FilePath> = files.collect::<Result<Vec<FilePath>, _>>()?;

        assert_eq!(first.file_name(), "a");
        let rest: Vec<&str> = rest.iter().map(|file| file.file_name()).collect();
        assert_eq!(rest, ["c"]);

        folder.delete_files()?;
        Ok(())
    }

    #[test]
    fn path_under_a_file_does_not_exist() -> Result<(), Box<dyn Error>> {
        let file: FilePath = FilePath::temp()?;
        file.write_empty()?;
        let under: FilePath = file.clone_append("/child").to_file()?;

        assert!(!under.exists()?);
        assert!(under.read_if_exists()?.is_none());
        assert!(under.read_as_vec_if_exists()?.is_none());
        assert!(!under.delete_if_exists()?);
        under.delete()?;

        file.delete()?;
        Ok(())
    }

    #[test]
    fn read_finds_no_file_at_a_folder() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;
        folder.clone_append("child").to_file()?.write_empty()?;
        let path: &str = folder.as_str().trim_end_matches('/');
        let file: FilePath = StoragePath::parse(path)?.to_file()?;

        assert!(file.read_if_exists()?.is_none());
        assert!(file.read_as_vec_if_exists()?.is_none());
        let reason: Reason = file.read_as_vec().unwrap_err().reason();
        assert_eq!(reason, Reason::FileNotFound);

        folder.delete_files()?;
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn read_rejects_a_device() -> Result<(), Box<dyn Error>> {
        // A device is neither a file nor a folder. Reads fail like `exists` does.
        let device: FilePath = StoragePath::parse("/dev/null")?.to_file()?;
        assert!(device.exists().is_err());
        assert!(device.read_if_exists().is_err());
        assert!(device.read_as_vec_if_exists().is_err());
        Ok(())
    }

    #[test]
    fn delete_removes_an_empty_folder() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;
        let nested: FilePath = folder.clone_append("a/b").to_file()?;
        nested.write_empty()?;
        nested.delete()?;

        // The empty folder `a` blocks a file `a` until it is deleted.
        let file: FilePath = folder.clone_append("a").to_file()?;
        assert!(file.write_empty().is_err());
        assert!(!file.delete_if_exists()?);
        file.write_empty()?;
        assert!(file.exists()?);

        file.delete()?;
        Ok(())
    }

    #[test]
    fn writes_fail_at_a_folder() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;
        folder.clone_append("a/b").to_file()?.write_empty()?;

        let file: FilePath = folder.clone_append("a").to_file()?;
        let errors: [file_storage::Error; 3] = [
            file.write_empty().unwrap_err(),
            file.write_if_not_exists()
                .err()
                .ok_or("write_if_not_exists found no folder")?,
            file.overwrite_with_headers("x", &Headers::new())
                .unwrap_err(),
        ];
        for error in errors {
            let message: String = error.to_string();
            assert!(
                message.contains("a folder is at the file path"),
                "{message}"
            );
        }

        folder.delete_files()?;
        Ok(())
    }

    #[test]
    fn delete_keeps_a_folder_with_files() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;
        let nested: FilePath = folder.clone_append("a/b").to_file()?;
        nested.write_empty()?;

        let file: FilePath = folder.clone_append("a").to_file()?;
        assert!(file.delete().is_err());
        assert!(nested.exists()?);

        folder.delete_files()?;
        Ok(())
    }

    #[test]
    fn delete_files_deletes_the_folders_it_empties() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;
        for name in ["a", "b/a", "b/c/d"] {
            folder.clone_append(name).to_file()?.write_empty()?;
        }

        folder.delete_files()?;
        assert!(Path::new(folder.as_str()).is_dir());
        assert!(!Path::new(folder.clone_append("b").as_str()).exists());

        // The folder itself is kept. Deleting its path as a file removes it.
        let path: &str = folder.as_str().trim_end_matches('/');
        StoragePath::parse(path)?.to_file()?.delete()?;
        assert!(!Path::new(folder.as_str()).exists());
        Ok(())
    }

    #[test]
    fn writes_leave_no_temp_files() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;
        let file: FilePath = folder.clone_append("file").to_file()?;

        drop(file.write()?);
        file.write_data("first")?;
        assert!(file.write_data("again").is_err());
        file.overwrite_with_headers("second", &Headers::new())?;

        assert_eq!(folder.list_files_as_vec()?, std::slice::from_ref(&file));
        assert_eq!(file.read_as_string()?, "second");

        folder.delete_files()?;
        Ok(())
    }

    #[test]
    fn exists_is_false_for_a_folder() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;
        folder.clone_append("a/b").to_file()?.write_empty()?;

        assert!(!folder.clone_append("a").to_file()?.exists()?);

        folder.delete_files()?;
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_are_followed_except_by_deletes() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

        let folder: FolderPath = FolderPath::temp()?;
        let outside: FolderPath = FolderPath::temp()?;
        let target: FilePath = outside.clone_append("data/x").to_file()?;
        target.write_data("x")?;
        let link = |name: &str| folder.clone_append(name);
        std::fs::create_dir_all(folder.as_str())?;
        symlink(target.as_str(), link("file").as_str())?;
        symlink(outside.clone_append("data").as_str(), link("dir").as_str())?;
        symlink(folder.as_str().trim_end_matches('/'), link("loop").as_str())?;
        let missing: StoragePath = outside.clone_append("missing");
        symlink(missing.as_str(), link("broken").as_str())?;

        // Listing follows links, skips the broken link, and cuts the loop.
        let files: Vec<FilePath> = folder.list_files_as_vec()?;
        let names: Vec<&str> = files.iter().map(|f| &f.as_str()[folder.len()..]).collect();
        assert_eq!(names, ["dir/x", "file"]);

        // Checking and reading follow links.
        let file: FilePath = link("file").to_file()?;
        assert!(file.exists()?);
        assert_eq!(file.read_as_string()?, "x");

        // Overwriting through a link replaces its target and keeps the link.
        file.overwrite_with_headers("z", &Headers::new())?;
        assert_eq!(target.read_as_string()?, "z");
        assert!(std::fs::symlink_metadata(file.as_str())?.is_symlink());

        // A create-only write through a broken link creates its target.
        let broken: FilePath = link("broken").to_file()?;
        assert!(!broken.exists()?);
        broken.write_data("y")?;
        let created: FilePath = missing.to_file()?;
        assert_eq!(created.read_as_string()?, "y");

        // Deleting removes the link itself, never its target.
        file.delete()?;
        assert!(!file.exists()?);
        assert!(target.exists()?);

        // Deleting the files removes the other links, never their targets.
        folder.delete_files()?;
        assert!(folder.list_files_as_vec()?.is_empty());
        assert!(target.exists()?);
        assert!(created.exists()?);

        outside.delete_files()?;
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn delete_files_keeps_a_folder_it_cannot_empty() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;
        let file: FilePath = folder.clone_append("b/a").to_file()?;
        file.write_empty()?;

        // A socket is not a file, so `delete_files` keeps it and its folder.
        let socket: StoragePath = folder.clone_append("b/socket");
        let listener: std::os::unix::net::UnixListener =
            std::os::unix::net::UnixListener::bind(socket.as_str())?;

        folder.delete_files()?;
        assert!(!file.exists()?);
        assert!(Path::new(folder.clone_append("b").as_str()).is_dir());

        // Listing skips the socket too.
        assert!(folder.list_files_as_vec()?.is_empty());

        drop(listener);
        std::fs::remove_file(socket.as_str())?;
        folder.delete_files()?;
        assert!(!Path::new(folder.clone_append("b").as_str()).exists());
        Ok(())
    }
}
