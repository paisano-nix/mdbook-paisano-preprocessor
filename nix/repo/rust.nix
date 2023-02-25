let
  inherit (inputs) fenix;
in
  # export fenix toolchain as it's own package set
  # change "default" to "complete" for nightly rust
  builtins.removeAttrs (fenix.packages.default // {inherit (fenix.packages) rust-analyzer;}) ["withComponents" "name" "type"]
