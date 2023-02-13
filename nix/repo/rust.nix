let
  inherit (inputs) fenix;
in
  # export fenix toolchain as it's own package set
  # change "stable" to "latest" for nightly rust
  builtins.removeAttrs (fenix.packages.latest // {inherit (fenix.packages) rust-analyzer;}) ["withComponents"]
