# Issues

## Backend Support

- [ ] Async API — async versions of all operations
- [ ] AWS S3 — native S3 support
- [ ] Google Cloud Storage
- [ ] Azure Blob Storage
- [ ] In-memory filesystem — useful for testing
- [ ] SFTP/SSH
- [ ] Custom backends / backend plugin system

## Operations

- [ ] Copy — `file.copy_to(dest)` across backends
- [ ] Move/Rename — `file.move_to(dest)`
- [ ] Metadata — file size, last modified, content type
- [ ] Create folder
- [ ] List folders
- [ ] Exists on folders
- [ ] Filtered listing — prefix, glob patterns
- [ ] Atomic write — write to temp then rename (local), conditional put (S3/R2)
- [ ] Append — append to existing file
- [ ] Partial read — read byte range

## Path

- [ ] Safe `StoragePath::new` — validated constructor without `unsafe`
- [ ] Borrowing `to_path` on `&self` for `FilePath`/`FolderPath`
- [ ] Path joining — `folder.join("sub/file.txt")`
- [ ] Path segment iteration
- [ ] Normalize — resolve `..` and `.` segments

## Quality

- [ ] Remove `once_cell` — migrate `R2_CLIENTS` to `LazyLock`
- [ ] Reduce unsafe usage
- [ ] Error improvements — typed error variants, better `source` chaining
- [ ] Logging/tracing — optional `tracing` integration
- [ ] Retry support — configurable retries for transient failures
- [ ] Connection pooling / client config — timeouts, concurrency limits
- [ ] `no_std` support — for the path module
- [ ] Fuzz testing — path parsing, edge cases
- [ ] Benchmarks — read/write throughput per backend
- [ ] CI for R2 tests — run ignored tests with credentials

## Ecosystem

- [ ] `serde` support — serialize/deserialize path types
- [ ] `tokio::io` traits — `AsyncRead`/`AsyncWrite` for streaming ops
- [ ] CLI tool — command line interface using the library
