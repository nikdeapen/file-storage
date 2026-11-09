use std::io;

/// The Unix error for a path whose parent is not a folder.
///
/// # Note
/// - `ErrorKind::NotADirectory` needs Rust 1.83. `ENOTDIR` is 20 on Linux, macOS and the BSDs.
const ENOTDIR: i32 = 20;

/// Checks if the `error` means nothing exists at the path.
///
/// # Note
/// - A path under a file fails with `ENOTDIR` on Unix. Nothing can exist there either.
pub(in crate::system::local) fn is_not_found(error: &io::Error) -> bool {
    error.kind() == io::ErrorKind::NotFound || (cfg!(unix) && error.raw_os_error() == Some(ENOTDIR))
}

#[cfg(test)]
mod tests {
    use super::{is_not_found, ENOTDIR};
    use std::io;
    use std::io::ErrorKind;

    #[test]
    fn is_not_found_matches_missing_paths() {
        assert!(is_not_found(&io::Error::from(ErrorKind::NotFound)));
        assert_eq!(
            is_not_found(&io::Error::from_raw_os_error(ENOTDIR)),
            cfg!(unix)
        );
        assert!(!is_not_found(&io::Error::from(ErrorKind::PermissionDenied)));
    }
}
