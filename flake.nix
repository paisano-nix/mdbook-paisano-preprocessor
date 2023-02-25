{
  description = "Yield documentation from Paisano-based projects into MdBook.";

  inputs = {
    std.url = "github:divnix/std";
    std.inputs.nixpkgs.follows = "nixpkgs";
    paisano-actions.url = "github:paisano-nix/actions";
    paisano-actions.inputs.nixpkgs.follows = "nixpkgs";
  };

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  inputs = {
    fenix.url = "github:nix-community/fenix";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    std,
    paisano-actions,
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
          }: [
            (paisano-actions.build currentSystem target)
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
