# Changelog

All notable changes to this project will be documented in this file

## [v2.0] - 2020-05-20
### Added
* Error Messages
* Override prompt and disabling options

### Changed
* Multi-threaded the watch mode
* Graceful exit on error number of arguments

### Removed
* Removed all stale parser functions

### Improved
* Improved the docs to reflect all major features
* Improved the README for better install instructions

# [v1.3] - 2020-05-05

### Added
* Added back variables
* Added the option to redirect the result to stdout

### Changed
* Chnaged the new variable syntax with the $ type syntax
* Changed the way where @ rules are put at the very bottom

### Removed
* Removed the old parsing engine

# [v1.2] - 2020-04-25
### Changed
* Rewrote the entire codebase in the Rust language

# [v1.1] - 2020-04-04

### Added
* AUR package for Arch Linux Users
* Quiet Mode: Pbss will not make any outputs when turned on
* Added Rept: For repeating objects
* Added checks to catch invalid CSS properties
* Added CONTRIBUTING and CHANGELOG

### Changed
* Changed documentation from markdown to Sphinx (.rst)
* Changed add function to attach function
* Attach function can take any number of files to join
* Fixed watch mode issues and put it in File class
* Fixed issues with media query and other at rules
* Rewrote the get_args function
* Changed the README to mostly refer to docs

### Removed
* Removed the add function
* Removed the old file watcher
* Removed Markdown files