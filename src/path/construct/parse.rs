#[cfg(feature = "r2")]
use crate::R2Path;
use crate::{Error, StoragePath};
use std::io;

impl StoragePath {
    //! Parse

    /// Parses a path string.
    ///
    /// # Formats
    /// - A unix path starts with `/`.
    /// - A Windows path starts with a drive like `C:\` or a UNC share like `\\server\share\`.
    /// - A Cloudflare R2 path starts with `https://{account}.r2.cloudflarestorage.com/{bucket}/`.
    ///   This is only enabled with the `r2` feature.
    ///
    /// # Errors
    /// - A path in none of these formats fails with `Unsupported`.
    /// - A Windows path that is incomplete or verbatim fails with `InvalidInput`.
    /// - With the `r2` feature, an invalid URL on the R2 host fails with `InvalidInput`.
    pub fn parse<S>(path: S) -> Result<Self, io::Error>
    where
        S: AsRef<str> + Into<String>,
    {
        let s: &str = path.as_ref();

        if Self::is_unix_path_str(s) {
            return Ok(unsafe { Self::new(path, 1, '/') });
        } else if Self::is_windows_path_str(s) {
            return Self::parse_windows(s);
        }

        #[cfg(feature = "r2")]
        if R2Path::is_r2_url(s) {
            return Self::parse_r2(path.into());
        }

        Err(Error::unknown_file_system(s))
    }

    /// Parses a Windows path.
    ///
    /// # Note
    /// - A UNC share without its last `\` gets one. Windows reads both as the share's root.
    fn parse_windows(s: &str) -> Result<Self, io::Error> {
        let mut path: String = s.chars().map(|c| Self::normal_char(c, '\\')).collect();
        let base_len: usize =
            Self::windows_base_len(&path).ok_or_else(|| Error::invalid_path(s))?;
        if path.len() < base_len {
            path.push('\\');
        }
        Ok(unsafe { Self::new(path, base_len, '\\') })
    }

    /// Gets the base length of a Windows path.
    ///
    /// # Contract
    /// - The file separators are already `\`.
    ///
    /// # Note
    /// - A UNC base counts the share's last `\`, even when the path leaves it off.
    /// - A UNC path without a server or share has no base.
    /// - Verbatim `\\?\` and device `\\.\` paths have no base. A `/` in them is not a separator.
    fn windows_base_len(path: &str) -> Option<usize> {
        let bytes: &[u8] = path.as_bytes();

        // A drive letter path like `C:\` has its first 3 bytes as the base.
        if bytes.len() >= 3
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && bytes[2] == b'\\'
        {
            return Some(3);
        }

        // A UNC path like `\\server\share\` has a base up to and including the share's separator.
        let (server, rest) = path.strip_prefix("\\\\")?.split_once('\\')?;
        let share: &str = rest.split_once('\\').map_or(rest, |(share, _)| share);
        if server.is_empty() || share.is_empty() || server == "?" || server == "." {
            return None;
        }
        Some(2 + server.len() + 1 + share.len() + 1)
    }

    /// Parses a Cloudflare R2 path.
    ///
    /// # Note
    /// - A bucket URL without its last `/` gets one. It is the bucket's root folder.
    #[cfg(feature = "r2")]
    fn parse_r2(mut path: String) -> Result<Self, io::Error> {
        if R2Path::base_len(&path).is_none() && R2Path::base_len(&format!("{path}/")).is_some() {
            path.push('/');
        }
        let base_len: usize = R2Path::base_len(&path).ok_or_else(|| Error::invalid_path(&path))?;
        Ok(unsafe { Self::new(path, base_len, '/') })
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;
    use std::io;
    use std::io::ErrorKind;

    #[test]
    fn parse_unix() {
        let path: StoragePath = StoragePath::parse("/folder/file.txt").unwrap();
        assert_eq!(path.base_path(), "/");
        assert_eq!(path.relative_path(), "folder/file.txt");
        assert_eq!(path.file_separator(), '/');
        assert!(path.is_file());
    }

    #[test]
    fn parse_unix_keeps_backslashes() {
        let path: StoragePath = StoragePath::parse("/a\\b").unwrap();
        assert_eq!(path.path(), "/a\\b");
        assert_eq!(path.relative_path(), "a\\b");
        assert!(path.is_file());
    }

    #[test]
    fn parse_drive_letter() {
        let path: StoragePath = StoragePath::parse("C:\\folder\\file.txt").unwrap();
        assert_eq!(path.path(), "C:\\folder\\file.txt");
        assert_eq!(path.base_path(), "C:\\");
        assert_eq!(path.relative_path(), "folder\\file.txt");
        assert_eq!(path.file_separator(), '\\');
        assert!(path.is_file());
    }

    #[test]
    fn parse_drive_letter_normalizes_separators() {
        let path: StoragePath = StoragePath::parse("C:/folder/file.txt").unwrap();
        assert_eq!(path.path(), "C:\\folder\\file.txt");
        assert_eq!(path.base_path(), "C:\\");
        assert_eq!(path.file_separator(), '\\');
    }

    #[test]
    fn parse_drive_letter_mixed_separators() {
        let path: StoragePath = StoragePath::parse("C:\\folder/file.txt").unwrap();
        assert_eq!(path.path(), "C:\\folder\\file.txt");
    }

    #[test]
    fn parse_drive_letter_folder() {
        let path: StoragePath = StoragePath::parse("C:\\folder\\").unwrap();
        assert!(path.is_folder());
    }

    #[test]
    fn parse_drive_letter_root() {
        let path: StoragePath = StoragePath::parse("C:\\").unwrap();
        assert_eq!(path.base_path(), "C:\\");
        assert_eq!(path.relative_path(), "");
        assert!(path.is_folder());
    }

    #[test]
    fn parse_unc_path() {
        let path: StoragePath = StoragePath::parse("\\\\server\\share\\file.txt").unwrap();
        assert_eq!(path.base_path(), "\\\\server\\share\\");
        assert_eq!(path.relative_path(), "file.txt");
        assert_eq!(path.file_separator(), '\\');
        assert!(path.is_file());
    }

    #[test]
    fn parse_unc_folder() {
        let path: StoragePath = StoragePath::parse("\\\\server\\share\\folder\\").unwrap();
        assert_eq!(path.base_path(), "\\\\server\\share\\");
        assert_eq!(path.relative_path(), "folder\\");
        assert!(path.is_folder());
    }

    #[test]
    fn parse_unc_root() {
        let path: StoragePath = StoragePath::parse("\\\\server\\share\\").unwrap();
        assert_eq!(path.base_path(), "\\\\server\\share\\");
        assert_eq!(path.relative_path(), "");
        assert!(path.is_folder());
    }

    #[test]
    fn parse_unc_share_without_separator() {
        let path: StoragePath = StoragePath::parse("\\\\server\\share").unwrap();
        assert_eq!(path.path(), "\\\\server\\share\\");
        assert_eq!(path.base_path(), "\\\\server\\share\\");
        assert_eq!(path.relative_path(), "");
        assert!(path.is_folder());

        let file: StoragePath = path.with_appended("file.txt");
        assert_eq!(file.path(), "\\\\server\\share\\file.txt");
        assert_eq!(file.relative_path(), "file.txt");
    }

    #[test]
    fn parse_unc_without_server_or_share_is_err() {
        for path in [
            "\\\\server",
            "\\\\server\\",
            "\\\\server\\\\file.txt",
            "\\\\/share\\file.txt",
        ] {
            let error: io::Error = StoragePath::parse(path).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput, "{path}");
        }
    }

    #[test]
    fn parse_verbatim_and_device_paths_are_err() {
        for path in [
            "\\\\?\\C:\\folder\\file.txt",
            "\\\\?\\UNC\\server\\share\\file.txt",
            "\\\\.\\COM1",
            "\\\\./PhysicalDrive0",
        ] {
            let error: io::Error = StoragePath::parse(path).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput, "{path}");
        }
    }

    #[test]
    fn parse_unknown_file_system() {
        for path in [
            "",
            "not_a_path",
            "C:",
            "C:file.txt",
            "\\folder",
            "https://example.com/bucket/",
        ] {
            let error: io::Error = StoragePath::parse(path).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Unsupported, "{path}");
        }
    }

    #[cfg(feature = "r2")]
    #[test]
    fn parse_r2() {
        let path: StoragePath =
            StoragePath::parse("https://account.r2.cloudflarestorage.com/bucket/folder/file.txt")
                .unwrap();
        assert_eq!(
            path.base_path(),
            "https://account.r2.cloudflarestorage.com/bucket/"
        );
        assert_eq!(path.relative_path(), "folder/file.txt");
        assert_eq!(path.file_separator(), '/');
        assert!(path.is_file());
    }

    #[cfg(feature = "r2")]
    #[test]
    fn parse_r2_bucket_without_slash() {
        let path: StoragePath =
            StoragePath::parse("https://account.r2.cloudflarestorage.com/bucket").unwrap();
        assert_eq!(
            path.as_str(),
            "https://account.r2.cloudflarestorage.com/bucket/"
        );
        assert_eq!(path.base_path(), path.as_str());
        assert!(path.is_folder());

        let file: StoragePath = path.with_appended("file.txt");
        assert_eq!(file.relative_path(), "file.txt");
    }

    #[cfg(feature = "r2")]
    #[test]
    fn parse_r2_invalid_is_invalid_input() {
        for path in [
            "https://account.r2.cloudflarestorage.com/Bucket/",
            "https://account.r2.cloudflarestorage.com/Bucket",
            "https://account.r2.cloudflarestorage.com//",
            "https://account.r2.cloudflarestorage.com",
            "https://.r2.cloudflarestorage.com/bucket/",
            "http://account.r2.cloudflarestorage.com/bucket/",
        ] {
            let error: io::Error = StoragePath::parse(path).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidInput, "{path}");
        }
    }

    #[cfg(not(feature = "r2"))]
    #[test]
    fn parse_r2_without_feature_is_unsupported() {
        let path: &str = "https://account.r2.cloudflarestorage.com/bucket/file.txt";
        let error: io::Error = StoragePath::parse(path).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Unsupported);
    }
}
