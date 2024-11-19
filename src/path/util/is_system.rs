/// Checks if the path is a local path.
pub fn is_local_path<S>(s: S) -> bool
where
    S: AsRef<str>,
{
    is_unix_path(s.as_ref()) || is_windows_path(s.as_ref())
}

/// Checks if the path is a Unix path.
pub fn is_unix_path<S>(s: S) -> bool
where
    S: AsRef<str>,
{
    s.as_ref().starts_with("/")
}

/// Checks if the path is a Windows path.
pub fn is_windows_path<S>(s: S) -> bool
where
    S: AsRef<str>,
{
    let s: &str = s.as_ref();
    if s.len() < 3 {
        false
    } else {
        s.as_bytes()[0].is_ascii_alphabetic()
            && s.as_bytes()[1] == b':'
            && (s.as_bytes()[2] == b'\\' || s.as_bytes()[2] == b'/')
    }
}
