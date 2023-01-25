# https://status.nixos.org (nixos-22.11)
{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/ab1254087f4c.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = builtins.attrValues {
    inherit (pkgs)
      bmake
      hugo;
  };
}
