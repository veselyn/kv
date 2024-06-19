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
      inherit (pkgs) lib;

      treefmtModule = treefmt.lib.evalModule pkgs ./treefmt.nix;

      jqSysDeps = with pkgs; [autoconf automake libtool];
      jqSysEnv = lib.optionalAttrs pkgs.stdenv.isDarwin {
        CPPFLAGS = "-D_REENTRANT";
      };

      tlsDeps = with pkgs;
        lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security
        ++ lib.optional stdenv.isLinux openssl;

      buildDeps = with pkgs; [pkg-config] ++ jqSysDeps;
      runtimeDeps = tlsDeps;
    in {
      formatter = treefmtModule.config.build.wrapper;

      packages = rec {
        devenv-up = self.devShells.${system}.default.config.procfileScript;

        default = kv;

        kv =
          (pkgs.rustPlatform.buildRustPackage {
            pname = "kv";
            version = "0.1.0";

            src = ./.;
            cargoLock = {lockFile = ./Cargo.lock;};

            cargoBuildsFlags = ["--package" "kv"];

            nativeBuildInputs = buildDeps;
            buildInputs = runtimeDeps;
          })
          .overrideAttrs
          jqSysEnv;

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
            packages = with pkgs;
              [
                git
                sea-orm-cli
                treefmtModule.config.build.wrapper
              ]
              ++ buildDeps
              ++ runtimeDeps;

            env = jqSysEnv;

            languages.rust = {
              enable = true;
            };

            pre-commit = {
              hooks = {
                clippy.enable = true;
                clippy.settings.denyWarnings = true;
                treefmt.enable = true;
                treefmt.package = treefmtModule.config.build.wrapper;
                test.enable = true;
                test.entry = "cargo test";
                test.pass_filenames = false;
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
