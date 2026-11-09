use file_storage::{FilePath, Headers, ReadOp, Reason, WriteOp};
use std::error::Error;
use std::io::{Read, Write};

/// Tests the operations on the `file`.
pub fn test_file(file: &FilePath, test_delete_if_exists: bool) -> Result<(), Box<dyn Error>> {
    exists(file)?;
    delete(file, test_delete_if_exists)?;
    read(file)?;
    read_string(file)?;
    read_vec(file)?;
    read_errors(file)?;
    write(file)?;
    write_nothing(file)?;
    write_if_not_exists(file)?;
    write_appears_at_close(file)?;
    write_loses_a_race_at_close(file)?;
    write_data(file)?;
    write_with_headers(file)?;
    Ok(())
}

/// Tests closing a `write` on the `file` without writing anything.
fn write_nothing(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    file.write()?.close()?;
    assert!(file.exists()?);
    assert!(file.read_as_vec()?.is_empty());

    Ok(())
}

/// Tests the `exists` function on the `file`.
fn exists(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    assert!(!file.exists()?);
    file.write_empty()?;
    assert!(file.exists()?);

    Ok(())
}

/// Tests the `delete` functions on the `file`.
fn delete(file: &FilePath, delete_if_exists: bool) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    assert!(!file.exists()?);
    file.write_empty()?;
    assert!(file.exists()?);

    file.delete()?;
    assert!(!file.exists()?);

    if delete_if_exists {
        assert!(!file.delete_if_exists()?);
        file.write_empty()?;
        assert!(file.delete_if_exists()?);
        assert!(!file.exists()?);
    }

    Ok(())
}

/// Tests the `read` functions on the `file`.
fn read(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    assert!(file.read_if_exists()?.is_none());
    file.write_data(s)?;

    let mut read: ReadOp = file.read()?;
    for b in s.as_bytes() {
        let buffer: &mut [u8] = &mut [0u8];
        read.read_exact(buffer)?;
        assert_eq!(*b, buffer[0]);
    }
    assert_eq!(read.read(&mut [0u8])?, 0);

    Ok(())
}

/// Tests the `read_string` functions on the `file`.
fn read_string(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    file.write_data(s)?;
    assert_eq!(file.read_as_string_if_exists()?.as_deref(), Some(s));

    let mut result: String = String::default();
    file.read_to_string(&mut result)?;
    assert_eq!(result, s);

    let mut result: String = "> ".to_string();
    assert_eq!(file.read_to_string(&mut result)?, s.len());
    assert_eq!(result, format!("> {s}"));

    Ok(())
}

/// Tests the `read_vec` functions on the `file`.
fn read_vec(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    assert!(file.read_as_vec_if_exists()?.is_none());
    file.write_data(s)?;
    assert_eq!(file.read_as_vec()?, s.as_bytes());

    let mut result: Vec<u8> = Vec::default();
    file.read_to_vec(&mut result)?;
    assert_eq!(result, s.as_bytes());

    let mut result: Vec<u8> = b"> ".to_vec();
    assert_eq!(file.read_to_vec(&mut result)?, s.len());
    assert_eq!(result, format!("> {s}").into_bytes());

    Ok(())
}

/// Tests the `read` functions on the `file` when they fail.
fn read_errors(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    // A missing file fails and leaves the target as it was.
    let error: file_storage::Error = file.read().err().ok_or("read found a missing file")?;
    assert_eq!(error.reason(), Reason::FileNotFound);
    let mut vec: Vec<u8> = b"keep".to_vec();
    let error: file_storage::Error = file.read_to_vec(&mut vec).unwrap_err();
    assert_eq!(error.reason(), Reason::FileNotFound);
    assert_eq!(vec, b"keep");
    let mut string: String = "keep".to_string();
    let error: file_storage::Error = file.read_to_string(&mut string).unwrap_err();
    assert_eq!(error.reason(), Reason::FileNotFound);
    assert_eq!(string, "keep");

    // Content that is not UTF-8 fails and leaves the target as it was.
    file.write_data([0xff, 0xfe])?;
    let error: file_storage::Error = file.read_as_string().unwrap_err();
    assert_eq!(error.reason(), Reason::FileContentNotUtf8);
    let error: file_storage::Error = file.read_to_string(&mut string).unwrap_err();
    assert_eq!(error.reason(), Reason::FileContentNotUtf8);
    assert_eq!(string, "keep");

    Ok(())
}

/// Tests the `write` function on the `file`.
fn write(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    let mut write: WriteOp = file.write()?;
    for b in s.as_bytes() {
        write.write_all(&[*b])?;
    }
    write.close()?;
    assert_eq!(file.read_as_string()?, s);

    Ok(())
}

/// Tests the `write_if_not_exists` function on the `file`.
fn write_if_not_exists(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    let write: Option<WriteOp> = file.write_if_not_exists()?;
    assert!(write.is_some());
    let mut write: WriteOp = write.unwrap();
    for b in s.as_bytes() {
        write.write_all(&[*b])?;
    }
    write.close()?;
    assert_eq!(file.read_as_string()?, s);

    assert!(file.write_if_not_exists()?.is_none());

    Ok(())
}

/// Tests the `write_data` function on the `file`.
fn write_data(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    file.write_data(s)?;
    assert_eq!(file.read_as_string()?, s);
    let error: file_storage::Error = file.write_data(s).unwrap_err();
    assert_eq!(error.reason(), Reason::FileAlreadyExists);

    Ok(())
}

/// Tests the `write_with_headers` and `overwrite_with_headers` functions on the `file`.
fn write_with_headers(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let headers: Headers = Headers::new().with_header("Content-Type", "text/plain");

    // An overwrite creates a missing file.
    file.overwrite_with_headers("new", &headers)?;
    assert_eq!(file.read_as_string()?, "new");
    file.delete()?;

    file.write_with_headers("first", &headers)?;
    let reason: Reason = file
        .write_with_headers("second", &headers)
        .unwrap_err()
        .reason();
    assert_eq!(reason, Reason::FileAlreadyExists);
    assert!(!file.write_with_headers_if_not_exists("second", &headers)?);
    assert_eq!(file.read_as_string()?, "first");

    file.overwrite_with_headers("second", &headers)?;
    assert_eq!(file.read_as_string()?, "second");

    Ok(())
}

/// Tests that a create-only write appears at `close` and a dropped one never does.
fn write_appears_at_close(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let mut write: WriteOp = file.write()?;
    write.write_all(b"data")?;
    assert!(!file.exists()?);
    drop(write);
    assert!(!file.exists()?);

    let mut write: WriteOp = file.write()?;
    write.write_all(b"data")?;
    write.close()?;
    assert_eq!(file.read_as_string()?, "data");

    Ok(())
}

/// Tests that a create-only write that loses a race fails at `close`.
fn write_loses_a_race_at_close(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    // Another writer creates the file while the streaming write is open.
    let mut write: WriteOp = file.write()?;
    write.write_all(b"late")?;
    file.write_data("first")?;
    let error: file_storage::Error = write.close().unwrap_err();
    assert_eq!(error.reason(), Reason::FileAlreadyExists);
    assert_eq!(file.read_as_string()?, "first");

    Ok(())
}
