{
  description = "kv";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils/main";
    devenv.url = "github:cachix/devenv/main";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    devenv,
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      packages = {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
      };

      devShells.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [];
      };
    });
}
