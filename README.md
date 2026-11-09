# file-storage

This library aids in accessing file-like storage.

## Features and Dependencies

```text
file-storage = "0.10.0"
```

### Primary Features

```text
r2          # enables Cloudflare R2 storage via the AWS SDK (uses tokio)
```

### Dependencies

Without features, the only dependency is `uuid`. It makes the random names of temp paths.

## Paths

Paths are parsed with `StoragePath::parse(path)`. The path format determines the file system.

- A path starting with `/` is a local Unix path.
- A path starting with a drive like `C:\`, or a UNC path like `\\server\share\`, is a local Windows
  path. Verbatim and device paths like `\\?\C:\` and `\\.\COM1` are not supported.
- A URL like `https://{account_id}.r2.cloudflarestorage.com/{bucket}/` is a Cloudflare R2 path. It
  needs the `r2` feature.

A UNC share or an R2 bucket written without its last separator gets one. `\\server\share` parses
as `\\server\share\`. An incomplete or invalid Windows path fails with `InvalidInput`. So does an
invalid URL on the R2 host, with the `r2` feature. Any other path fails with `Unsupported`.

Local paths only work on a matching host. A Windows path on Unix fails with `UnknownFileSystem`, as
does a Unix path on Windows.

A `StoragePath` is split into a base path and a relative path. The base path is the root of the
file system, such as `/`, `C:\` or `https://account.r2.cloudflarestorage.com/bucket/`. A path is a
`FolderPath` if it equals its base path or ends with the file separator. Otherwise it is a
`FilePath`.

Windows paths always use `\` as the file separator. Parsing and appending turn each `/` into `\`.
A path built by appending equals the same path parsed in one piece. Unix and R2 paths keep `\` as
is.

Paths compare by their exact text. Letter case matters on Windows too.

`FilePath::temp()` and `FolderPath::temp()` give a random path in the system's temp folder. Nothing
is created on disk. `FolderPath::current_working_directory()` gives the working directory.

## Operations

Operations are called directly on the path. They dispatch to the right backend. No file system
object or client needs to be created or managed.

`FilePath` supports these operations.

- `exists`
- `read`, `read_as_string`, `read_to_string`, `read_as_vec` and `read_to_vec`
- `write`, `write_data`, `write_empty` and `write_with_headers`
- `overwrite_with_headers`
- `delete`

`FolderPath` supports `list_files` and `delete_files`. Both cover every file beneath the folder.

Most file operations also have an `_if_exists` or `_if_not_exists` form. These are atomic. A
backend that cannot do one atomically returns `UnsupportedOperation`. Cloudflare R2 cannot delete
atomically. Its `delete_if_exists` is unsupported.

`write` opens a `WriteOp` to stream a new file. The file only appears when `close` succeeds.
Dropping the `WriteOp` without calling `close` discards the write. If another writer creates the
file first, `close` fails with `FileAlreadyExists`.

`write_with_headers` and `overwrite_with_headers` take `Headers`, such as `Content-Type`. Cloudflare
R2 stores them with the object. The local file system ignores them.

## Errors

Every operation returns an `Error`. It holds the path, the `Operation`, a `Reason` and sometimes an
IO source. It converts into an `io::Error` with a matching `ErrorKind`.

`Display` shows the IO source. `source()` skips it and returns the IO source's own source. An error
chain prints the IO source once. Use `io_source` to get it.

## Local File Systems

Local symlinks are followed when checking, reading, listing and creating files. A link back to a
folder being listed is skipped. Deletes remove the link itself and never touch its target.

A local folder at a file path is not a file. `exists` returns false, reads find no file and writes
fail. Entries that are neither files nor folders, such as sockets, are skipped by listing and
deleting.

A local write goes to a hidden temp file in the same folder until `close` moves it into place. A
create-only write links it to the file. The link fails if the file exists. An overwrite renames it
over the file. Readers never see part of a file. A crash can leave a temp file behind, named
`.file-storage-{pid}-{n}.tmp`. Creating a file needs hard links. File systems without them, such
as FAT, cannot create files.

An overwrite makes a new file. It has default permissions. Hard links to the old file keep the old
data.

## Cloudflare R2

R2 clients load the default AWS credentials, such as `AWS_ACCESS_KEY_ID` and
`AWS_SECRET_ACCESS_KEY`. An R2 token belongs to one account. A process can only reach that one
account.

R2 operations block on an internal tokio runtime. Calling one from inside an async runtime panics.

A missing bucket is an error. `exists` is the exception. A HEAD cannot tell a missing bucket from a
missing file. It returns false.

Keys that end with `/` are folder markers. Listing skips them and `delete_files` deletes them.
`delete_files` on a bucket's root folder deletes every object in the bucket.

A streaming write sends parts of 5 MiB. An object can hold at most 10,000 parts. That is about
48.8 GiB.
