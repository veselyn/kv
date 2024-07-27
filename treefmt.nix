_: {
  projectRootFile = "flake.nix";

  programs = {
    alejandra.enable = true;
    rustfmt.enable = true;
    prettier.enable = true;
  };
}
