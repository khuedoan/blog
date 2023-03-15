# https://status.nixos.org (nixos-22.11)
{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/67f26c1cfc5d.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = builtins.attrValues {
    inherit (pkgs)
      gnumake
      nodejs;
  };
}
