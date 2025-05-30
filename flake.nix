# SPDX-FileCopyrightText: 2023 Christina Sørensen
# SPDX-FileContributor: Christina Sørensen
#
# SPDX-License-Identifier: AGPL-3.0-only
{
  description = "rime:  Nix Flake Input Versioning";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    treefmt-nix,
    pre-commit-hooks,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};

        naersk' = naersk.lib.${system};

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;

        nativeBuildInputs = with pkgs; [pkg-config];
        buildInputs = with pkgs; [openssl];
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

            nativeBuildInputs = with pkgs; [makeWrapper installShellFiles] ++ nativeBuildInputs;

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
            inherit nativeBuildInputs buildInputs;
          };

          # Run `nix build .#test` to run tests
          test = naersk'.buildPackage {
            src = ./.;
            mode = "test";
            inherit nativeBuildInputs buildInputs;
          };

          # Run `nix build .#clippy` to lint code
          clippy = naersk'.buildPackage {
            src = ./.;
            mode = "clippy";
            inherit nativeBuildInputs buildInputs;
          };
        };

        # For `nix develop`:
        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
          nativeBuildInputs = with pkgs; [rustup toolchain just zip reuse vhs fish] ++ nativeBuildInputs;
          inherit buildInputs;
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
    )
    // {
      nixosModules = {
        default = self.nixosModules.rime;
        rime = nixpkgs.lib.modules.importApply ./module.nix {inherit self;};
      };
    };
}
