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
> RIME is experimental, and its API is subject to change.

## What is rime?

As Nix and Nix adjacent projects have historically had terrible documentation,
we'll be extra clear here: Rime is a decentralized, one to many FOSS AGPLv3
REUSE compliant middleware REST API for flake inputs.

Just kidding! All that is true, but what rime is actually about is simple. Nix
has these things called flakes. Flakes can take other flakes as inputs.
Usually, you add an input to a flake by adding a link to a git repo. The repo
you link, will then be locked to the specific version at that time.

But that's kinda weird if you think about it. You don't usually download the
latest development version of a program do you? No! If you're using a
distribution, like nixpkgs, then some people have already added a specific
version of that program. And not just any version, you get the latest version
that the developer of that program (upstream) has released.

Flakes can't do that, at least not natively. So you get this annoying situation
that when you update your flake inputs with `nix flake update`, you get some
random, unstable development version. This design choice leads to a lot of
problem, both for user who now are forced to wait for their favourite software
to be added to nixpkgs (and updated for that matter), but also for developers
like me, who run a project like eza that relies on flakes. While flakes
themselves are awesome, not having versioned releases for your inputs... isn't
at all.

So why doesn't flakes do that? Bureaucracy, partially at least, but as we've
come to realize by making rime, there are actually a lot of advantages to it
not doing that, because that allows us to do it in a much better way.

## Why rime?

You see, what rime offers is this: you specify a software forge, like github,
as well as a user and a repository, and we'll make sure that you get the latest
released version from the developer whenever it is ready.

Sounds too good to be true? There's more! We also support a LOT of forges, like:

- Github
- Gitlab (Including self hosted)
- Codeberg
- Forgejo (Including self hosted)
- Gitea (Including self hosted)
- Sourcehut (Including self hosted, we think at least >_<)
- Flakehub (Only tagged versions, for now)

...and we plan to add support for cgit, cgit-pink, and any other forge you can
think of (create a feature request issue)!

This saves users and developers a lot of headaches. And to make it even better,
we also support automatically discovering what kind of forge is at an url, and
plan to support semver requirements, like only getting releases that aren't
major changes, or only getting those between version `2.3.2` and `2.8.9`.

Also just like, how would you add a input from sourcehut, or a flakehub one? 
No need to look it up, rimes got you covered!

## How rime???

And it gets even better, because rime is very simple, and you could easily run
your own version. And if that's not your cup of tea, or you're a developer that
needs a hosted version that "just works", we also have rime.cx, our own
kubernetes cluster hosting rime, with plenty of beef to handle all your
requests. And if you wanna make your own cluster with a rime deployment, we
provided an example of how to do that as well, and a ghcr.io container as part
of the flake.

Enough talking, wanna see it in action?

## Please show me how to use this already!

It's as easy as this:

```
nix run http://rime.cx/v1/github/cafkafk/fortune-kind.tar.gz
```

That will run the latest version of
[fortune-kind](https://github.com/cafkafk/fortune-kind) that I have personally
released. Neat hu?

Of course, you'll need to have `flakes` and the `nix-command` [experimental features](https://nixos.org/manual/nix/stable/contributing/experimental-features)
enabled. But don't worry, they're not really “experimental”, that's mostly a
result of bureaucracy. If you wanna reassure yourself, determinate systems, 
a company founded by the guy that created ofborg, and currently employs the 
guy that created Nix in the first place [has this to say about flakes stability](https://determinate.systems/posts/experimental-does-not-mean-unstable).

## What about flake inputs that don't do releases

Two options, one is to use one of rimes endpoints for branches, versions, or
tags, like this:

```
nix run http://rime.cx/v1/codeberg.org/cafkafk/hello/version/v0.0.1.tar.gz
nix run http://rime.cx/v1/codeberg.org/cafkafk/hello/tag/v0.0.1.tar.gz
nix run http://rime.cx/v1/codeberg.org/cafkafk/hello/branch/main.tar.gz
```

Another option is to use [semnix](https://github.com/semnix), our collection of essential flake inputs that
we ourselves need for development, that we release ourselves.

That means you can have a flake like this:

```nix
{
  description = "Your awesome flake";

  inputs = {
    {
  description = "rime:  Nix Flake Input Versioning";

  inputs = {
    flake-utils.url = "http://rime.cx/v1/github/semnix/flake-utils.tar.gz";

    naersk.url = "http://rime.cx/v1/github/semnix/naersk.tar.gz";
      
    treefmt-nix.url = "http://rime.cx/v1/github/semnix/treefmt-nix.tar.gz";

    rust-overlay.url = "http://rime.cx/v1/github/semnix/rust-overlay.tar.gz";

    ... your other inputs
  }

  outputs = {
    ... all your cool outputs
  }
}
```

# Wow, that's really cool, how do I help this

Well... [donations](https://github.com/sponsors/cafkafk) to cover hosting costs are always much appreciated.

If you wanna become a rime hacker (and why wouldn't you), I'd like to make you familiar with two very important documents.
Firstly, you're expected to follow our [code of conduct](https://github.com/cafkafk/rime/blob/main/CODE_OF_CONDUCT.md).
After that, you can have a look at our [CONTRIBUTING.md](https://github.com/cafkafk/rime/blob/main/CONTRIBUTING.md) for more info about actual hacking. Also, while silly branch names are preferred, you're free to name them something professional, we won't judge.
