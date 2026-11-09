# Issues

## Cloudflare R2

- Per-account R2 credentials — every account's client loads the same default AWS credentials
- Larger R2 streaming writes — `write` caps at ~48.8 GiB (10,000 parts of 5 MiB); needs a size hint or bigger parts

## Path

- Windows verbatim and device paths — `\\?\C:\...` and `\\.\...` are rejected by `parse`

## Testing

- CI for R2 tests — run ignored tests with credentials
- CI for Windows — the build only runs on Linux, so Windows-only code is never compiled
