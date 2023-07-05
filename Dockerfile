FROM nixos/nix

WORKDIR /app

COPY flake.nix flake.nix

RUN nix develop --extra-experimental-features 'nix-command flakes' --command echo

COPY . .

RUN make
