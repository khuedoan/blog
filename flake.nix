{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs =  import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          packages = [
            binaryen
            cargo-leptos
            gnumake
            imagemagick
            leptosfmt
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            (nodePackages.tailwindcss.overrideAttrs (attrs: {
              plugins = [
                nodePackages."@tailwindcss/typography"
              ];
            }))
          ];
        };
      }
    );
}
