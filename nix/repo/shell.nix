let
  lib = nixpkgs.lib // builtins;
  inherit (inputs) nixpkgs std;
in
  lib.mapAttrs (_: std.lib.dev.mkShell) rec {
    default = {
      name = "Paisano MdBook";
      nixago = [
        ((std.lib.dev.mkNixago std.lib.cfg.conform)
          {data = {inherit (inputs) cells;};})
        ((std.lib.dev.mkNixago std.lib.cfg.treefmt)
          cell.config.treefmt)
        ((std.lib.dev.mkNixago std.lib.cfg.editorconfig)
          cell.config.editorconfig)
        ((std.lib.dev.mkNixago std.lib.cfg.githubsettings)
          cell.config.githubsettings)
        (std.lib.dev.mkNixago std.lib.cfg.lefthook)
        (std.lib.dev.mkNixago std.lib.cfg.adrgen)
      ];
      packages = [
        nixpkgs.pkg-config
      ];
      commands =
        map (name: {
          inherit name;
          package = cell.rust.toolchain; # has all bins
          category = "rust dev";
          # fenix doesn't include package descriptions, so pull those out of their equivalents in nixpkgs
          help = nixpkgs.${name}.meta.description;
        }) [
          "rustc"
          "cargo"
          "rustfmt"
          "rust-analyzer"
        ];

      imports = [
        #  cell.shell.book
        book
        "${std.inputs.devshell}/extra/language/rust.nix"
      ];

      language.rust = {
        packageSet = cell.rust;
        tools = ["toolchain"]; # fenix collates them all in a convenience derivation
        enableDefaultToolchain = false;
      };

      devshell.startup.link-cargo-home = {
        deps = [];
        text = ''
          # ensure CARGO_HOME is populated
          mkdir -p $PRJ_DATA_DIR/cargo
          ln -f -s -t $PRJ_DATA_DIR/cargo $(ls -d ${cell.rust.toolchain}/*)
        '';
      };

      env = [
        {
          # ensure subcommands are picked up from the right place
          # but also is writable
          name = "CARGO_HOME";
          eval = "$PRJ_DATA_DIR/cargo";
        }
        {
          # ensure cargo caches to .std/rustup
          name = "RUSTUP_HOME";
          eval = "$PRJ_DATA_DIR/rustup";
        }
        {
          name = "RUST_SRC_PATH";
          # accessing via toolchain doesn't fail if it's not there
          # and rust-analyzer is graceful if it's not set correctly:
          # https://github.com/rust-lang/rust-analyzer/blob/7f1234492e3164f9688027278df7e915bc1d919c/crates/project-model/src/sysroot.rs#L196-L211
          value = "${cell.rust.toolchain}/lib/rustlib/src/rust/library";
        }
        {
          name = "PKG_CONFIG_PATH";
          value = lib.makeSearchPath "lib/pkgconfig" inputs.cells.app.package.mdbook-paisano-preprocessor.buildInputs;
        }
      ];
    };

    book = {
      nixago = [
        ((std.lib.dev.mkNixago std.lib.cfg.mdbook) cell.config.mdbook)
      ];
    };
  }
