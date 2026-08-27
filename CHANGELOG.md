# Changelog

All relevant updates will be listed in this document

## Unreleased

### Added

- Ability to skip configured `output_file` and `pretty` settings in the config file via the CLI

## 3.0.0

### Updated

- Name to be an optional value in the snipe targets configuration file

### Fixed

- Bug where missing variables and environment variables can be displayed more than once in error messages

### Added

- Dry run
- Layered config/CLI arguments
- Variable interpolation system for auth, headers, and payload
- Shortcut to pretty print JSON output
- Redaction of of potential sensitive values in logs
- Better logging
- Better formatting for single field responses
- Logic to ignore comments in configuration file when doing variable interpolation
- Sorting for `list` command

## 2.0.0

### Added

- Slight tweaks to CLI ergonomics

## 1.2.0

### Added

- Ability to upload files as response body

## Version 1.1.2

### Fixed

- Release process

## Version 1.1.1

### Fixed

- Release pipeline

## 1.1.0

### Fixed 

- `.pre-commit` hooks not failing for warnings

### Refactored

- Internal code to use `ValidatedGrab` notation
- Internal code to use `ValidatedFormat` notation

### Updated

- `README` file

### Added

- Better release pipeline
- Ability to write responses to binary files
- `CODEOWNERS` file
- `LICENSE` file

## Version 1.0.0

### Added

- Initial working implementation
