# TODO support pure Nix container build instead of Dockerfile
# I'm to lazy to make static binary to work with both Nix & Dockerfile at the same time
FROM nixos/nix

RUN echo 'experimental-features = flakes nix-command' >> /etc/nix/nix.conf \
    && echo 'filter-syscalls = false' >> /etc/nix/nix.conf

WORKDIR /app

COPY . .
RUN nix develop --command make

EXPOSE 3000

CMD ["/app/target/release/blog"]
