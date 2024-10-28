Today I noticed I haven't reinstalled my [NixOS](https://nixos.org) system in
over a year:

```sh
$ sudo tune2fs -l /dev/nvme0n1p2 | grep 'Filesystem created'
Filesystem created:       Sun Jul 16 17:48:32 2023
```

And it’s been rock-solid ever since. I almost never feel like the system is
"dirty" or in need of reinstallation.

This has never happened before since I started using Linux as my daily driver.
I’ve never managed to go more than a year without reinstalling Arch, Ubuntu,
Fedora, or any other system. It’s not that my Arch system _actually_ has a
problem that requires a complete reinstall; it just _feels_ "dirty" after a few
months. I have some kind of "OCD" that keeps nudging me to reinstall the system
to restore that clean state.

## NixOS always _feels_ clean

In NixOS, I always have to update my [NixOS config
repository](https://github.com/khuedoan/nixos-setup) to make any changes, no
manual changes are permitted.

The main reason my Arch system feels dirty is that it’s easy to lose discipline
with Configuration as Code (CaC) tools, which leads me to run manual commands
to install random packages or test new configurations.

Sometimes I remember to add these changes to my Ansible playbooks, sometimes I
don’t. And even if I do, Ansible can fail between tasks, making it hard to
fully roll back a change. This brings me to the next point.

## NixOS is truly declarative

Every change in NixOS is built in isolation within the Nix store. Once a build
completes, the system switches **atomically** to the new version via symlinks.
Tools like Ansible, Puppet, and Chef try to be declarative, but they’re
essentially a series of imperative steps. If a playbook stops halfway or
encounters an error, you’re left with a half-old, half-new system. In NixOS,
the switch only occurs after the new system is fully built, and it does so
atomically. Rollbacks are just a symlink change away or can be done by
selecting an older bootloader entry.

To be fair, NixOS isn’t 100% declarative and atomic. For example, service
activation and restarts are still imperative steps after the full system
switch, but it’s pretty damn close to being fully declarative and atomic.

The only system that offers the same capabilities, to my knowledge, is [GNU
Guix](https://guix.gnu.org). However, I'm sticking with Nix because I don't
have a reason to switch yet, and Nix seems to have more packages available.

## Nix can isolate per-project dependencies

Almost all of my personal projects now have a `flake.nix` file that
automatically sets up the development environment for that specific project.
Any change required to get a project running for local development is scoped to
that `flake.nix` file, so I don't have to mess around with my "sacred" system
configuration repository. I can also trust the Nix garbage collector to clean
up dependencies that I don't use after a while.

This is the best of both worlds: my base system is always clean, while I can
still have almost any software I want for each project.
