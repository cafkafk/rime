<!--
SPDX-FileCopyrightText: 2023 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: AGPL-3.0-only
-->

# Changelog

All notable changes to this project will be documented in this file.

## [0.1.7] - 2023-12-07

### Bug Fixes

- Axum 0.7.2 breaking changes

### Miscellaneous Tasks

- Bump clap from 4.4.8 to 4.4.10
- Bump axum from 0.6.20 to 0.7.2

## [0.1.6] - 2023-11-30

### Features

- Add integration testing against freedesktops gitlab

### Miscellaneous Tasks

- Bump serde from 1.0.192 to 1.0.193
- Bump form_urlencoded from 1.2.0 to 1.2.1
- Bump hyper from 0.14.27 to 1.0.1
- Bump openssl from 0.10.57 to 0.10.60
- Release rime v0.1.6

## [0.1.5] - 2023-11-23

### Bug Fixes

- Remove redundant comment

### Miscellaneous Tasks

- Update flake inputs
- Bump http-body-util from 0.1.0-rc.3 to 0.1.0
- Release rime v0.1.5

### Testing

- Update itest-live

## [0.1.4] - 2023-11-16

### Miscellaneous Tasks

- Bump tokio from 1.33.0 to 1.34.0
- Bump clap from 4.4.7 to 4.4.8
- Bump serde from 1.0.190 to 1.0.192
- Release rime v0.1.4

### Refactor

- Make ForgeReleases.matching more functional

## [0.1.3] - 2023-11-09

### Documentation

- Update the comment about url encoding

### Features

- Support including pre-releases
- Add flagship routes for (git.)sr.ht
- Add an endpoint for semver locking
- Allow more complex semver locking
- Add support for release-based endpoints
- Try extracting a semver more aggressively

### Miscellaneous Tasks

- Add more treefmt'ers, precommit hack
- Bump serde_json from 1.0.107 to 1.0.108
- Release rime v0.1.3

### Refactor

- Refactor the release discovery
- Configurable forge API page size
- Lift out the release list fetching
- Lift out a lot of shared code
- Use flagship routes for Codeberg
- Greatly simplify the handlers
- Turn auto-discovery into a "Forge"
- Lift routing out of the Forge trait
- Drop `new()` from the Forge trait
- Allow some shortcuts for implementors
- Lift out the .tar.gz stripping
- Use the original URI for NoTarGz
- Re-export ForgeReleases from api::v1::forge
- Lift out some shared code

### Testing

- Add testing for pre-releases
- Codeberg must now pass
- Add testing for the semver endpoint
- Add testing for the release endpoint
- Temporarily disable gitlab.com autodiscovery

### Bug

- Improved auto-detection

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
- Release rime v0.1.2

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

