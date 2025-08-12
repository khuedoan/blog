{
  inputs = {
    # TODO rustc in NixOS stable is not new enough
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
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
          binaryen
          cargo
          cargo-leptos
          cargo-nextest
          clippy
          k6
          leptosfmt
          lld
          rustc
          rustfmt
        ] ++ lib.optional stdenv.isDarwin [
          libiconv
        ];
      };
    });
  };
}
