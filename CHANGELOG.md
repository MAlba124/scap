# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.6](https://github.com/MAlba124/scap/compare/v0.0.5...v0.0.6) - 2024-06-01

### Added
- adds correct crop_area
- get_crop_area for specific targets
- adds scale_factor support for windows and displays on mac
- get_main_display func improved
- add unique identifier to unknown displays on mac
- adds correct name of displays on macos
- make scale_factor f64
- exclude windows without title
- adds windows as targets on mac
- restructure util functions and add display name windows

### Fixed
- use cg types from sckit_sys
- output frame size target
- windows tweaks
- macos imports after restructure

### Other
- cleanup deps and remove cgtype in favor of area
- update readme and add todo for windows
- Merge branch 'feat/solo-target' into feat/use-targets-mac
- Merge pull request [#84](https://github.com/MAlba124/scap/pull/84) from helmerapp/feat/use-targets-windows
- Merge branch 'feat/solo-target' into feat/mac-targets-scale-factor
- Merge pull request [#81](https://github.com/MAlba124/scap/pull/81) from helmerapp/feat/windows-improvements
- Merge branch 'main' into feat/windows-targets
- Merge branch 'feat/windows-targets' of https://github.com/helmerapp/scap into feat/windows-targets
- extract pixelformat conversions to different file
- source rect simplifier
- shorten width, height
- windows engine
- tweak example app
- updates readme

## [0.0.5](https://github.com/helmerapp/scap/compare/v0.0.4...v0.0.5) - 2024-05-25

### Other
- don't build before releasing
- remove CHANGELOG
