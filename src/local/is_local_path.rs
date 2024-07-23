pub fn is_local_path<S>(path: S) -> bool
where
    S: AsRef<str>,
{
    is_unix_path(path.as_ref()) || is_windows_path(path.as_ref())
}

pub fn is_unix_path<S>(path: S) -> bool
where
    S: AsRef<str>,
{
    let path: &[u8] = path.as_ref().as_bytes();
    !path.is_empty() && path[0] == b'/'
}

pub fn is_windows_path<S>(path: S) -> bool
where
    S: AsRef<str>,
{
    let path: &[u8] = path.as_ref().as_bytes();
    !path.is_empty()
        && path[0].is_ascii_uppercase()
        && path[1] == b':'
        && (path[2] == b'/' || path[2] == b'\\')
}
