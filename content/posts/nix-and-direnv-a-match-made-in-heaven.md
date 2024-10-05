- Don't need to think
- Extremely fast with (almost) zero overhead
  - Using nix-direnv instead of the built in implementation
- Works without container, no abstraction, no VM on macOS

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

The NixOS one will use nix-direnv by default, which is much faster.
