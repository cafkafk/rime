<!--
SPDX-FileCopyrightText: 2023 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: AGPL-3.0-only
-->
<div align="center">

<h1>RIME</h1>

![Rimed hexagonal snow crystal under electron microscope](docs/images/rime.jpg)

Nix Flake Input Versioning

[![Built with Nix](https://img.shields.io/badge/Built_With-Nix-5277C3.svg?logo=nixos&labelColor=73C3D5)](https://nixos.org)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)
[![Unit tests](https://github.com/cafkafk/rime/actions/workflows/test.yml/badge.svg)](https://github.com/cafkafk/rime/actions/workflows/test.yml)
[![REUSE status](https://api.reuse.software/badge/git.fsfe.org/reuse/api)](https://api.reuse.software/info/git.fsfe.org/reuse/api)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

</div>

> **Warning**
> RIME is experimental, and its API is subject to change. You're warned.

# Rime

Rime is an open-source middleware, designed to bridge the gap between Nix Flakes and Git Source Forges. Licensed under AGPLv3, it offers a REST API interface that is both REUSE compliant and free of any push action requirements from repositories – every repo is inherently part of Rime. 

Unlike some alternatives like flakehub, Rime stands out by not mandating any push actions from the repository. Moreover, Rime currently operates without retaining any state, focusing primarily on logic that facilitates redirection through its API. Please note that the API is in a developmental phase and might be unstable.

As Rime is in in an early phase of development, it only supports github, but aims to support major forges as well as other places you might put a flake, whatever they may be. Rime is not a fan of github defaultism.

(Notice that Rime doesn't aim to be a flakehub replacement, flakehub and Rime have different approaches with different tradeoffs)

## Key Features:

- **Built with Rust/Axum:** Rime is crafted using the Rust programming language and the Axum web framework.
  
- **Flakes-First Approach:** Designed with a priority on Nix Flakes, ensuring seamless integration with them and Git Source Forges.

- **Auto-Generates Essential Documents:** Rime can automatically generate completions and man pages, enhancing the usability.

- **Configurable:** Customize Rime using a dedicated configuration file to suit your needs.

- **Decentralized:** Rime is not monolithic. Use well-known instances like `rime.cx` (hosted by me) or set up your personal instance. You have the flexibility to activate your Rime instance as required, such as during system rebuilds.

![A technical demo gif based on the script in docs/tapes/demo.tape](docs/images/demo.gif)

## Get Started

    nix run github:cafkafk/rime
