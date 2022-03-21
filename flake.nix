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
      in
      {
        packages.cargo-workspaces = pkgs.cargo-workspaces;
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [ libiconv anchor-0_22_0 cargo-workspaces ]
            ++ (lib.optionals stdenv.isDarwin
              (with darwin.apple_sdk.frameworks; [ AppKit IOKit Foundation ]));
        };
      });
}
