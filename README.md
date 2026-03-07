# file-storage

This library aids in accessing file-like storage.

## Features & Dependencies

    file-storage = "0.8.0-rc.1"

### Primary Features

    tempfile    # enables temporary local path creation
    r2          # enables cloudflare r2 storage via the aws sdk (uses tokio)

### Dependencies

This crate has no dependencies unless features are enabled.

## Usage

Paths are parsed with `StoragePath::parse(path)`. The path format determines the file system — a
path starting with `/` uses the local Unix filesystem, `C:\` uses Windows, and an R2 URL dispatches
to Cloudflare R2. All operations (`read`, `write`, `delete`, `list_files`, etc.) are called directly
on the path and dispatch to the correct backend automatically. No file system object or client needs
to be created or managed.

A `StoragePath` is split into a base path and a relative path. The base path identifies the root of
the file system (e.g. `/`, `C:\`, or `https://account.r2.cloudflarestorage.com/bucket/`). A path is
a `FolderPath` if it equals its base path or ends with the file separator, otherwise it is a
`FilePath`.

`FilePath` supports: `exists`, `read`, `write`, `write_data`, `delete`.

`FolderPath` supports: `list_files`, `delete_files`.
