# Changelog

All notable changes to this project will be documented in this file.

## [0.3.2] - 2023-11-01

[264847e](264847e2a93dcc2c32db7f7c16cb3209c53a9172)...[0eedf15](0eedf15acbd7da0f27a612efdf6d6b97ec751349)

### Bug Fixes

- Pass a `ref` instead ([535ba3b](535ba3b1ec6504b8975d714a3ab9c05097132eaa))

### Documentation

- Add missing doc for some methods ([3302c0e](3302c0ea2df073a2623edd8deef7053f55f67d98))

### Features

- Add `git-clif` config for better changelog ([58bb86b](58bb86b0f42d0d9a65b5cfe2a0f34f4ef1c31b1d))
- Expose a `PipelineTrait` trait ([fbc2bc9](fbc2bc9d7cd7ae701b7405be004029c8ea484f35))
- Add `PipelineTrait` implementation example ([528f1fd](528f1fd7fb18129378d1117736f4eb0b233d260a))

## [0.3.2] - 2023-11-01

### Bug Fixes

- Pass a `ref` instead

### Documentation

- Add missing doc for some methods

### Features

- Add `git-clif` config for better changelog
- Expose a `PipelineTrait` trait
- Add `PipelineTrait` implementation example


## Version 0.3.1

- Add the ability to return Option<T>. When T is none, the pipe flow will stop. The returned value is also stored in the pipeline
- Add the ability to return and store Result<T, E>. When result is `err` the pipe flow will be stopped

## Version 0.3.0

Pipe handlers can now specify returned value. This allows you to use general function or struct instead
of specific ones. 

## Version 0.2.0

The entire crate was reimplemented. `Busybody` is use for dependency injecting.