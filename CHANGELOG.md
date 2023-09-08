# Changelog
All notable changes to `traitful` will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [0.2.1] - 2023-09-08
### Added
 - Support bounds on generics (`#[seal]`, `#[extend]`)
 - Support `for<>` syntax in `#[extend]`
 - Support `#[extend]` with no attribute params

## [0.2.0] - 2023-07-23
### Changed
 - `#[extend]` API to operate on traits rather than impls
 - Improved documentation for `#[extend]`
 - Improved documentation for `#[seal]`

## [0.1.0] - 2023-07-23
### Added
 - `#[seal]` attribute macro
 - `#[extend]` attribute macro

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://github.com/AldaronLau/semver/blob/stable/README.md
