{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
  };

  outputs = { nixpkgs, ... }:
  {
    devShells = builtins.mapAttrs (system: pkgs: {
      default = with pkgs; mkShell {
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
        ] ++ lib.optionals stdenv.isDarwin [
          libiconv
        ];
      };
    }) nixpkgs.legacyPackages;
  };
}
