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
      checks.ci = self.devShells.${system}.default.config.ciDerivation;
      formatter = treefmtModule.config.build.wrapper;

      packages = {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
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
                treefmt.enable = true;
              };
              settings = {
                clippy.denyWarnings = true;
                treefmt.package = treefmtModule.config.build.wrapper;
              };
            };
          }
        ];
      };
    });
}
