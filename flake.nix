{
  inputs.fenix.url = "github:nix-community/fenix";

  outputs = { fenix, ... }: {
    devShells = fenix.inputs.nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ] (system: {
      default = fenix.inputs.nixpkgs.legacyPackages.${system}.mkShell {
        packages = [ fenix.packages.${system}.stable.toolchain ];
      };
    });
  };
}
