{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs }:
  let
    supportedSystems = nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
    ];
  in
  {
    devShells = supportedSystems (system: {
      default = with nixpkgs.legacyPackages.${system}; mkShell {
        packages = [
          bacon
          cargo
          cargo-nextest
          clippy
          k6
          rustc
          rustfmt
        ] ++ lib.optional stdenv.isDarwin [
          libiconv
        ];
      };
    });
  };
}
