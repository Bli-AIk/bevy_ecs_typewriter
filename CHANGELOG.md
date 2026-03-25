# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/Bli-AIk/bevy_ecs_typewriter/compare/v0.2.0...v0.2.1) - 2026-03-25

### Added

- *(ci)* add tokei lint checks to crate workflows

### Miscellaneous Tasks

- *(lint)* improve #[expect] reason detection in tokei scripts
- add clippy configuration
- *(crates)* add readme and repository fields to Cargo.toml files

### Refactor

- *(plugin)* split runtime wiring and enforce lint thresholds ([#12](https://github.com/Bli-AIk/bevy_ecs_typewriter/pull/12))
- *(examples)* remove clippy expect attribute from sync systems
- *(deps)* update bevy dependencies to disable default features
- *(examples)* replace clippy allow with expect attributes

## [0.2.0](https://github.com/Bli-AIk/bevy_ecs_typewriter/compare/v0.1.1...v0.2.0) - 2026-02-11

### Added

- [**breaking**] upgrade to bevy 0.18

## [0.1.1](https://github.com/Bli-AIk/bevy_ecs_typewriter/compare/v0.1.0...v0.1.1) - 2026-02-06

### Miscellaneous Tasks

- update CI workflows and gitignore patterns ([#6](https://github.com/Bli-AIk/bevy_ecs_typewriter/pull/6))
- add gitignore files for bevy crates
