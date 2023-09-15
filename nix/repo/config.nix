let
  inherit (inputs) nixpkgs;
  inherit (inputs.std.data) configs;
  inherit (inputs.std.lib.dev) mkNixago;
in {
  treefmt = (mkNixago configs.treefmt) {
    data.formatter = {
      rust = {
        command = "rustfmt";
        includes = ["*.rs"];
      };
    };
    packages = [
      cell.rust.toolchain
    ];
  };
  editorconfig = (mkNixago configs.editorconfig) {};
  conform = (mkNixago configs.conform) {};
  lefthook = (mkNixago configs.lefthook) {};

  mdbook = (mkNixago configs.mdbook) {
    data.book.title = "Paisano MdBook Preprocessor";
  };

  cog = (mkNixago configs.cog) {
    data.changelog = {
      remote = "github.com";
      repository = "paisano-nix";
      owner = "mdbook-paisano-preprocessor";
    };
  };

  githubsettings = (mkNixago configs.githubsettings) {
    data = {
      repository = {
        name = "mdbook-paisano-preprocessor";
        inherit (import (inputs.self + /flake.nix)) description;
        homepage = "https://paisano-nix.github.io/mdbook-paisano-preprocessor";
        topics = "nix, std, hive";
        default_branch = "main";
        allow_squash_merge = true;
        allow_merge_commit = false;
        allow_rebase_merge = true;
        delete_branch_on_merge = true;
        has_projects = false;
        has_wiki = false;
        has_download = false;
      };
      milestones = [
        {
          title = "Documentation";
          description = ":dart:";
          state = "open";
        }
      ];
    };
  };
}
