let
  lib = nixpkgs.lib // builtins;
  inherit (inputs) nixpkgs std;
in
  lib.mapAttrs (_: std.lib.dev.mkShell) rec {
    default = {
      name = "Paisano MdBook";
      nixago = [
        (std.lib.cfg.conform {data = {inherit (inputs) cells;};})
        (std.lib.cfg.treefmt cell.config.treefmt)
        (std.lib.cfg.editorconfig cell.config.editorconfig)
        (std.lib.cfg.githubsettings cell.config.githubsettings)
        std.lib.cfg.lefthook
        std.lib.cfg.adrgen
      ];
      packages = [
        nixpkgs.pkg-config
      ];
      commands = let
        rustCmds =
          lib.mapAttrs' (name: package: {
            inherit name;
            value = {
              inherit package name;

              category = "mdbook-dev";
              # fenix doesn't include package descriptions, so pull those out of their equivalents in nixpkgs
              help = nixpkgs.${name}.meta.description;
            };
          }) {
            inherit
              (cell.rust)
              cargo
              clippy # also has: rustc
              rustfmt
              rust-analyzer
              ;
          };
      in
        [
        ]
        ++ lib.attrValues rustCmds;

      imports = [
        #  cell.shell.book
        book
        "${std.inputs.devshell}/extra/language/rust.nix"
      ];

      language.rust = {
        packageSet = cell.rust.toolchain;
        enableDefaultToolchain = false;
      };

      env = [
        {
          name = "RUST_SRC_PATH";
          value = "${cell.rust.rust-src}/lib/rustlib/src/rust/library";
        }
        # {
        #   name = "PKG_CONFIG_PATH";
        #   value = lib.makeSearchPath "lib/pkgconfig" inputs.cells.app.package.mdbook-paisano-preprocessor.buildInputs;
        # }
      ];
    };

    book = {
      nixago = [
        (std.lib.cfg.mdbook cell.config.mdbook)
      ];
    };
  }
