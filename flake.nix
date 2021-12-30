{
  description = "Saber Vipers development environment.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    saber-overlay.url = "github:saber-hq/saber-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, saber-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; }
          // saber-overlay.packages.${system};
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [ libiconv anchor-0_19_0 cargo-workspaces ];
        };
      });
}
