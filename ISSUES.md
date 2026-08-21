# Issues

## Backend Support

- Async API — async versions of all operations
- AWS S3 — native S3 support
- Google Cloud Storage
- Azure Blob Storage
- In-memory filesystem — useful for testing
- SFTP/SSH
- Custom backends / backend plugin system

## Operations

- Copy — `file.copy_to(dest)` across backends
- Move/Rename — `file.move_to(dest)`
- Metadata — file size, last modified, content type
- Create folder
- List folders
- Exists on folders
- Filtered listing — prefix, glob patterns
- Atomic write — write to temp then rename (local), conditional put (S3/R2)
- Append — append to existing file
- Partial read — read byte range
- Surface R2 delete_files per-object errors — DeleteObjects 200s can report per-key failures; needs Error changes
- Skip folder-marker keys in R2 list_files — keys ending `/` (dashboard-created) currently error as non-file paths
- Make R2 streaming write_if_not_exists atomic — HEAD-then-create races; use If-None-Match on complete-multipart
- Remove the partial file when a local create-only write fails — retries currently see it and return Ok(false)
- Fix write_with_headers overwrite semantics — it silently overwrites while sibling write ops are create-only
- Fix unknown R2 write headers silently becoming x-amz-meta-* — extend the typed whitelist or reject unknown names
- Fix local overwrite durability — fs::write truncates in place (old data lost on failure) & skips sync_all
- Define a Headers duplicate/case policy — get is first-match case-insensitive while R2 puts are last-wins, case-kept
- Signal ignored headers on local write_with_headers — non-empty Headers are silently dropped

## Path

- Safe `StoragePath::new` — validated constructor without `unsafe`
- Borrowing `to_path` on `&self` for `FilePath`/`FolderPath`
- Path joining — `folder.join("sub/file.txt")`
- Path segment iteration
- Normalize — resolve `..` and `.` segments
- Validate the R2 account id & bucket when parsing — an id holding `/` puts an arbitrary host in the signed endpoint

## Quality

- Reduce unsafe usage
- Error improvements — typed error variants, better `source` chaining
- Logging/tracing — optional `tracing` integration
- Retry support — configurable retries for transient failures
- Connection pooling / client config — timeouts, concurrency limits
- `no_std` support — for the path module
- Fuzz testing — path parsing, edge cases
- Benchmarks — read/write throughput per backend
- CI for R2 tests — run ignored tests with credentials
- Document backend gaps — R2 delete_if_exists is unsupported, R2 ops panic inside async runtimes, symlinks skipped
- Update the README — capability list omits write_with_headers & the Headers type
- Replace the dashmap 7.0.0-rc2 pre-release pin — stable dashmap or a locked HashMap
- Run rustfmt over the crate & add a `cargo fmt --check` gate to CI (13 files currently fail)
- Build the no-features configuration in CI — the feature matrix only covers tempfile & r2
- Fix the Headers derive list — wrong order & missing Ord/PartialOrd/Hash
- Impl Debug as Display for the path types & add #[must_use] to the value types
- Deduplicate write code — write_with_headers_if_not_exists local branch & repeated R2 error-wrapping arms
- Avoid per-header allocations in R2 put_request — compare with eq_ignore_ascii_case
- Remove the dead symlink branch in local exists — fs::metadata follows symlinks so it never fires

## Ecosystem

- `serde` support — serialize/deserialize path types
- `tokio::io` traits — `AsyncRead`/`AsyncWrite` for streaming ops
- CLI tool — command line interface using the library
