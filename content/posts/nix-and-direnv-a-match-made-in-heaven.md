# Nix and direnv - a match made in heaven

This is just a quick post to praise the combination of [Nix](https://nixos.org)
and [direnv](https://direnv.net).

Here's how my current setup looks (using the `tofu` program as an example, it
works for any program available in
[nixpkgs](https://search.nixos.org/packages)):

```sh
❯ tofu version
tofu: command not found

❯ cd ~/Documents/homelab

❯ tofu version
OpenTofu v1.7.1
on linux_amd64
```

Did you notice? I didn't have to do anything to get the correct version of
`tofu` in my shell. I just `cd` into the project, and it's there.

Okay, I lied a little. I did have to do something to get this to work. I have a
`flake.nix` file in my project directory that defines all dependencies for the
project. Almost all of my projects now have a `flake.nix` file, and it looks
something like this:

```nix
# Truncated for brevity
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs }:
    # ...
      {
        devShells.default = mkShell {
          packages = [
            # The list of dependencies for this project
            kubectl
            opentofu
            go
            # ...
          ];
        };
      }
    );
}
```

And in my `.envrc` file:

```sh
use flake
```

You can think of this as a `Cargo.toml` or `package.json` file, but for all CLI
tools.

This is the magic of `nix` and `direnv`:

- All dependencies are managed by a per-project `flake.nix` file.
- No need to worry about different versions of dependencies for different
  projects.
- Extremely fast thanks to
  [`nix-direnv`](https://github.com/nix-community/nix-direnv) instead of the
  default `direnv` implementation.
- Works without containers or abstractions - just a `$PATH` manager on steroids!

To set them up, assuming you're using NixOS:

```nix
{
  programs = {
    direnv = {
      enable = true;
      silent = true;
    };
  };
}
```

NixOS already uses `nix-direnv` by default, so you don't have to enable it
explicitly.
