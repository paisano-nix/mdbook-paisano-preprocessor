{
  description = "Yield documentation from Paisano-based projects into MdBook.";

  inputs = {
    std.url = "github:divnix/std";
    std.inputs.nixpkgs.follows = "nixpkgs";
    std.inputs.devshell.follows = "devshell";
    std.inputs.nixago.follows = "nixago";
    devshell.url = "github:numtide/devshell";
    devshell.inputs.nixpkgs.follows = "nixpkgs";
    nixago.url = "github:nix-community/nixago";
    nixago.inputs.nixpkgs.follows = "nixpkgs";
    nixago.inputs.nixago-exts.follows = "";
  };

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  inputs = {
    fenix.url = "github:nix-community/fenix";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    std,
    self,
    ...
  } @ inputs:
    std.growOn {
      inherit inputs;
      cellsFrom = ./nix;
      cellBlocks = with std.blockTypes; [
        {
          name = "package";
          type = "binary";
          actions = {
            currentSystem,
            fragment,
            fragmentRelPath,
            target,
            inputs,
          }: [
            (std.actions.build currentSystem target)
          ];
        }

        {
          name = "rust";
          type = "binary";
          cli = false;
        }

        # Contribution & Documentation Environment
        (devshells "shell")
        (nixago "config")
      ];
    }
    {
      packages = std.harvest self ["app" "package"];
    };
}
