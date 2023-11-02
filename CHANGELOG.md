<!--
SPDX-FileCopyrightText: 2023 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: AGPL-3.0-only
-->

# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2023-11-02

### Bug Fixes

- Add SystemConfiguration, refactor slightly

### Documentation

- Fix typo
- Revamped readme

### Features

- Add sourcehut version endpoints
- Add ref endpoint
- Branch endpoint

### Miscellaneous Tasks

- Release rime v0.1.1
- Bump flake inputs
- Gracefully handle partial configs

### Testing

- Add integration tests for the live server
- Add testing of slashes in branches

### Bug

- Sanitize the tarball name in get_repo_ref
- Handle branch names with slashes in them
- Fix Forgejo & GitLab latest tag APIs

### Build

- Change all flake inputs to rime.cx
- Change most flake inputs to semnix

## [0.1.1] - 2023-10-30

### Documentation

- Create SECURITY.md
- Add CONTRIBUTING.md
- Fix broken markdown link

### Features

- Make root route redirect to github repo for now
- Introduce flakehub api endpoint (version only)
- Introduce self-hosted forgejo api endpoint
- Introduce codeberg api endpoint
- Add a gitea route
- Introduce a self-hosted gitlab endpoint
- Try auto-discovering forges
- Direct mapping for github.com & flakehub.com

### Miscellaneous Tasks

- Add dependabot.yml
- Bump serde from 1.0.188 to 1.0.190
- Bump regex from 1.9.5 to 1.10.2
- Add REUSE headers to SECURITY.md, dependabot.yml
- Bump serde_yaml from 0.9.25 to 0.9.27
- Make `cafkafk` codeowner of everything

### Refactor

- Fix oversight in suffix check
- Deny unwrap_used in clippy
- Remove commented out code
- Move routing out into tree structure
- Use nested routing

### Testing

- Add simple integration tests for most forges
- Add autodiscovery integration tests

### Build

- Make cross compilation skip uncompilable
- Automate release of tagged containers

## [0.1.0] - 2023-10-28

### Features

- Add CHANGELOG.md

### Miscellaneous Tasks

- Make rime public
- Release rime v0.1.0

