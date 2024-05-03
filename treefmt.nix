{pkgs, ...}: {
  projectRootFile = "flake.nix";

  programs = {
    alejandra.enable = true;
    rustfmt.enable = true;
    prettier.enable = true;
  };
  settings = {
    formatter = {
      sqlfluff = {
        command = "${pkgs.sqlfluff}/bin/sqlfluff";
        options = ["format" "--dialect=sqlite"];
        includes = ["*.sql"];
      };
    };
  };
}
