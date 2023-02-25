let
  crane = inputs.crane.lib.overrideToolchain inputs.cells.repo.rust.toolchain;
in {
  default = cell.package.mdbook-paisano-preprocessor;
  mdbook-paisano-preprocessor = crane.buildPackage {
    src = inputs.std.incl inputs.self [
      "src"
      "assets"
      "templates"
      "Cargo.lock"
      "Cargo.toml"
    ];
  };
}
