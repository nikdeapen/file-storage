# Contributing

This guide outlines how contributions should be made to the codebase.

## Commits

Each commit is one small, focused change. Its message is a tag with one sentence. Example:

```text
[write] Added support for writing headers.
```

There shouldn't be a description.

### Tags

The tag denotes the section of the codebase the commit is relevant to.

The crate:

- `[*]`: changes across multiple parts of the crate.
- `[cargo]`: the crate, typically anything focused outside the `src` folder.

The sections:

- `[path]`: the storage paths.
- `[error]`: the errors.
- `[util]`: the utilities, such as the headers.
- `[test]`: the tests in the `tests` folder.

The operations:

- `[exists]`: checking if a file exists.
- `[read]`: reading files.
- `[write]`: writing files.
- `[delete]`: deleting files.
- `[list-files]`: listing the files in a folder.
- `[delete-files]`: deleting the files in a folder.
