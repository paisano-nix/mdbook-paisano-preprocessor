let
  inherit (inputs) fenix;

  # change "stable" to "[minimal|default|complete|latest]" for nightly rust
  rustPkgs = fenix.packages.default;
  rustPkgs' =
    if rustPkgs ? rust-analyzer
    then rustPkgs
    else rustPkgs // {inherit (fenix.packages) rust-analyzer;};
in
  # export fenix toolchain as it's own package set
  builtins.removeAttrs rustPkgs' ["withComponents" "name" "type"]
