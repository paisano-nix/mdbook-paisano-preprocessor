let
  inherit (inputs) nixpkgs;
  lib = nixpkgs.lib // builtins;
in {
  treefmt = {
    data = {
      formatter = {
        nix = {
          command = "alejandra";
          includes = ["*.nix"];
        };
        prettier = {
          command = "prettier";
          options = ["--plugin" "prettier-plugin-toml" "--write"];
          includes = [
            "*.css"
            "*.html"
            "*.js"
            "*.json"
            "*.jsx"
            "*.md"
            "*.mdx"
            "*.scss"
            "*.ts"
            "*.yaml"
            "*.toml"
          ];
        };
        shell = {
          command = "shfmt";
          options = ["-i" "2" "-s" "-w"];
          includes = ["*.sh"];
        };
        prettier = {
          excludes = ["**.min.js"];
        };
        rust = {
          command = "rustfmt";
          includes = ["*.rs"];
        };
      };
    };
    packages = [
      nixpkgs.alejandra
      nixpkgs.nodePackages.prettier
      nixpkgs.nodePackages.prettier-plugin-toml
      nixpkgs.shfmt
      cell.rust.toolchain
    ];
    devshell.startup.prettier-plugin-toml = lib.stringsWithDeps.noDepEntry ''
      export NODE_PATH=${nixpkgs.nodePackages.prettier-plugin-toml}/lib/node_modules:''${NODE_PATH:-}
    '';
  };
  editorconfig = {
    hook.mode = "copy"; # already useful before entering the devshell
    data = {
      root = true;

      "*" = {
        end_of_line = "lf";
        insert_final_newline = true;
        trim_trailing_whitespace = true;
        charset = "utf-8";
        indent_style = "space";
        indent_size = 2;
      };

      "*.{diff,patch}" = {
        end_of_line = "unset";
        insert_final_newline = "unset";
        trim_trailing_whitespace = "unset";
        indent_size = "unset";
      };

      "*.md" = {
        max_line_length = "off";
        trim_trailing_whitespace = false;
      };
      "{LICENSES/**,LICENSE}" = {
        end_of_line = "unset";
        insert_final_newline = "unset";
        trim_trailing_whitespace = "unset";
        charset = "unset";
        indent_style = "unset";
        indent_size = "unset";
      };
    };
  };
  mdbook = {
    output = "docs/book.toml";
    data = {
      book = {
        language = "en";
        multilingual = false;
        title = "Paisano MdBook Preprocessor";
        src = ".";
      };
      build = {
        build-dir = "book";
      };
    };
    hook.mode = "copy"; # let CI pick it up outside of devshell
  };
  githubsettings = {
    data = {
      repository = {
        name = "mdbook-paisano-preprocessor";
        inherit (import (inputs.self + /flake.nix)) description;
        homepage = "https://paisano-nix.github.io/mdbook-paisano-preprocessor";
        topics = "nix, nix-flakes";
        default_branch = "main";
        allow_squash_merge = true;
        allow_merge_commit = false;
        allow_rebase_merge = true;
        delete_branch_on_merge = true;
        has_projects = false;
        has_wiki = false;
        has_download = false;
      };
    };
  };
}
