{
  description = "kv";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils/main";
    devenv.url = "github:cachix/devenv/main";
    treefmt.url = "github:numtide/treefmt-nix/main";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    devenv,
    treefmt,
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      treefmtModule = treefmt.lib.evalModule pkgs ./treefmt.nix;
    in {
      formatter = treefmtModule.config.build.wrapper;

      packages = {
        devenv-up = self.devShells.${system}.default.config.procfileScript;

        default = pkgs.rustPlatform.buildRustPackage {
          pname = "kv";
          version = "0.1.0";
          src = ./.;
          cargoLock = {lockFile = ./Cargo.lock;};
        };

        redisjson = pkgs.rustPlatform.buildRustPackage rec {
          pname = "redisjson";
          version = "2.6.10";

          src = pkgs.fetchFromGitHub {
            owner = "redisjson";
            repo = "redisjson";
            rev = "v${version}";
            hash = "sha256-zWYhA0gKEUvxiq/kLb34vTVnaHu73CF8OYCWQ7NfPtM=";
          };

          cargoLock = {
            lockFile = "${src}/Cargo.lock";
            outputHashes = {
              "ijson-0.1.3" = "sha256-GFNNGsXWXS3BWsYffxhAnWtPh7rboGWJ1FmSHSidNmI=";
            };
          };
        };
      };

      devShells.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          {
            packages = with pkgs; [
              treefmtModule.config.build.wrapper
              refinery-cli
            ];

            languages.rust = {
              enable = true;
            };

            pre-commit = {
              hooks = {
                clippy.enable = true;
                clippy.settings.denyWarnings = true;
                treefmt.enable = true;
                treefmt.package = treefmtModule.config.build.wrapper;
              };
            };

            services = {
              redis.enable = true;
              redis.extraConfig = "loadmodule ${self.packages.${system}.redisjson}/lib/librejson.dylib";
            };

            processes = with pkgs; {
              readme.exec = "${python3Packages.grip}/bin/grip";
            };
          }
        ];
      };
    });
}
