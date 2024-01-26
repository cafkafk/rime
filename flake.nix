# SPDX-FileCopyrightText: 2023 Christina Sørensen
# SPDX-FileContributor: Christina Sørensen
#
# SPDX-License-Identifier: AGPL-3.0-only
{
  description = "rime:  Nix Flake Input Versioning";

  inputs = {
    flake-utils.url = "http://rime.cx/v1/github/semnix/flake-utils.tar.gz";
    naersk.url = "http://rime.cx/v1/github/semnix/naersk.tar.gz";
    nixpkgs.url = "http:/rime.cx/v1/github/NixOS/nixpkgs/b/nixpkgs-unstable.tar.gz";
    treefmt-nix.url = "http://rime.cx/v1/github/semnix/treefmt-nix.tar.gz";
    rust-overlay.url = "http://rime.cx/v1/github/semnix/rust-overlay.tar.gz";

    pre-commit-hooks = {
      url = "http://rime.cx/v1/github/semnix/pre-commit-hooks.nix.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    treefmt-nix,
    rust-overlay,
    pre-commit-hooks,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;

        darwinBuildInputs = with pkgs; with darwin.apple_sdk.frameworks; [libiconv Security SystemConfiguration];

        buildInputs = with pkgs; [pkg-config openssl] ++ lib.optionals stdenv.isDarwin darwinBuildInputs;
      in rec {
        # For `nix fmt`
        formatter = treefmtEval.config.build.wrapper;

        packages = {
          # For `nix build` & `nix run`:
          default = naersk'.buildPackage {
            src = ./.;
            doCheck = true; # run `cargo test` on build
            copyBins = true;
            copyLibs = true;
            singleStep = true;
            inherit buildInputs;

            nativeBuildInputs = with pkgs; [makeWrapper installShellFiles];

            MAN_OUT = "./man";

            preBuild = ''
              mkdir -p "./$MAN_OUT";
            '';

            preInstall = ''
              installManPage man/rime.1
              installShellCompletion \
                --fish man/rime.fish \
                --bash man/rime.bash \
                --zsh  man/_rime
              mkdir -p $out
            '';
          };

          container = pkgs.dockerTools.buildLayeredImage {
            name = "rime";
            tag = "latest";
            contents = [packages.default pkgs.cacert];
            config = {
              Labels = {
                "org.opencontainers.image.source" = "https://github.com/cafkafk/rime";
                "org.opencontainers.image.description" = "rime:  Nix Flake Input Versioning";
                "org.opencontainers.image.license" = "AGPL-3.0-only";
              };
              Env = [
                "RUST_LOG=trace"
              ];
              Cmd = ["/bin/rime"];
            };
          };

          # Run `nix build .#check` to check code
          check = naersk'.buildPackage {
            src = ./.;
            mode = "check";
            inherit buildInputs;
          };

          # Run `nix build .#test` to run tests
          test = naersk'.buildPackage {
            src = ./.;
            mode = "test";
            inherit buildInputs;
          };

          # Run `nix build .#clippy` to lint code
          clippy = naersk'.buildPackage {
            src = ./.;
            mode = "clippy";
            inherit buildInputs;
          };
        };

        # For `nix develop`:
        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
          nativeBuildInputs = with pkgs; [rustup toolchain just zip reuse pkg-config openssl vhs fish];
        };

        # For `nix flake check`
        checks = {
          pre-commit-check = let
            # some treefmt formatters are not supported in pre-commit-hooks we filter them out for now.
            toFilter =
              # HACK: This is a nice hack to not have to manually filter we should keep in mind for a future refactor.
              # Stolen from eza
              ["yamlfmt"];
            filterFn = n: _v: (!builtins.elem n toFilter);
            treefmtFormatters = pkgs.lib.mapAttrs (_n: v: {inherit (v) enable;}) (pkgs.lib.filterAttrs filterFn (import ./treefmt.nix).programs);
          in
            pre-commit-hooks.lib.${system}.run {
              src = ./.;
              hooks =
                treefmtFormatters
                // {
                  convco.enable = true; # not in treefmt
                  reuse = {
                    enable = true;
                    name = "reuse";
                    entry = with pkgs; "${pkgs.reuse}/bin/reuse lint";
                    pass_filenames = false;
                  };
                };
            };
          formatting = treefmtEval.config.build.check self;
          build = packages.check;
          inherit
            (packages)
            default
            test
            ;
          lint = packages.clippy;
        };
      }
    );
}
