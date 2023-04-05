let
  crane = inputs.crane.lib.overrideToolchain inputs.cells.repo.rust.toolchain;
  inherit (inputs.nixpkgs) stdenv lib;
in {
  default = cell.package.mdbook-paisano-preprocessor;
  mdbook-paisano-preprocessor = crane.buildPackage {
    buildInputs = with inputs.nixpkgs; [libiconv];
    src = inputs.std.incl inputs.self [
      "src"
      "assets"
      "templates"
      "Cargo.lock"
      "Cargo.toml"
    ];
  };
}
