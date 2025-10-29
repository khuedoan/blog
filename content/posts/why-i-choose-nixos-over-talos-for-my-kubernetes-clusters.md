# Why I choose NixOS over Talos for my Kubernetes clusters

DRAFT

There are many things that I like in Talos that are missing in NixOS
(admittedly I haven't tried Talos again since https://github.com/khuedoan/homelab/pull/2 with Sidero Metal & PXE Talos
install, so I might have missed some new improvements):

- Talos is very minimal (80MB!!!) thus less attack surface and remarkably fast
  install (NixOS + k3s can be made into a small foot print with relatively fast
  install, but not as tiny and fast)
- An actual controlplane to coordinate day 2 operations, not a bunch of shell
  scripts or Ansible playbooks to do things like "upgrade master nodes first,
  drain before rebooting etc.". And holy shit controlling bare metal machines
  with Sidero Cluster API is nice once everything is set up
- They also have a separate PXE [booter](https://github.com/siderolabs/booter) CLI now that can be used without
Sidero Metal

But using NixOS I'll have to sacrifies those things (and may have to reinvent
some of them) for other advantages:

1. The first one is non-technical: with all due respect to the amazing and hard
work of the Sidero Labs team, I have more peace of mind that NixOS will be
around for a long time and continue to be open-source, given its 19-year
history and the community is only getting bigger. I'm not saying Sidero Labs
will definitely relicense or deprecate Talos, but there's a risk of that as
demonstrated by the deprecation of the open-source Sidero Metal (MPL2) in favor
of the source-available Omni (BSL, which unlike Sidero Metal, Omni runs on a
separate machine). I understand that they need to balance open source with
business revenue, which is really, really hard, but that's also why I worry
about having a core part of my software stack depend on a single VC-funded
company.

2. With that being said there are other technical things that I think NixOS is
more suitable for my use case. The long term goal for this project is a "mom we
have Heroku at home" homelab, not nessesarily a k8s homelab. I may switch from
k8s to something else if there's a better alternative, but the OS layer should
be very stable and rarely changes, which means I’ll lean forward a general
purpose distro instead of ones that are built specifically for k8s - but of
course k8s is a very high bar that I don't think there's a better alternative
anytime soon [^1]. Due to this reason some of the next points
might be biased towards general purpose distros as well

3. NixOS unlocks some vertical integrations for Nix-based CI/CD:
    - Use the node's Nix store to speed up my CI/CD running inside Nix shell,
      which manages per-project dependencies (e.g. no need for `setup-python`
      like GitHub Actions). Maybe add hook to upload built artifact to a global
      Nix cache so that I never have to build something twice again (e.g. if a
      package/container was built on the PR branch, it doesn't need to be
      rebuilt after merging, and the cache works at the individual package
      level and across repos)
    - [nix-snapshotter](https://github.com/pdtpartners/nix-snapshotter) for efficient containers and the ability for transparent source-based
      deployment
    - Higher level Nix abstractions such as [devenv](https://devenv.sh) or [Nixpacks](https://github.com/railwayapp/nixpacks) might be able to enjoy
      the same optimizations

4. NixOS can do most (all?) Talos can do regarding reproducibility, ephemerality,
declarativity, atomic upgrade and more:
    - Talos is only declarative at runtime config, and Talos OS image builds
      are not (easily) reproducible. NixOS is declarative from source to build
      to runtime config. In Nix binary deployment is “just” a transparent
      optimization for source-based deployment [^2]. If I have things cached in
      the Nix store of the laptop running the installation, I basically get
      air-gapped base cluster install for free, and other apps on top of it if I try
      hard enough to preload all images tgz to the configuration, as opposed to
      Talos where I need a separate private container registry
    - While NixOS is not ephemeral by default, it can additionally be configured
      in an [erase your darlings](https://grahamc.com/blog/erase-your-darlings) style to match Talos' ephemeral design
    - Talos atomic upgrades work by using A/B partitions, NixOS atomic upgrades
      work by switching symlinks in Nix store which can have as many versions
      as my disk allows, and it can be switched without reboot to test the
      configuration (except the kernel), and if A/B partition is desired it can
      be configured with systemd-repart for OTA-style upgrade [^2] as well.

6. Talos mostly doesn't have the single system (node) deployment problem to
solve since it's A/B partition-based at the cost of easily customize the OS.
Nix/NixOS while still customizable has **really** good foundation and solved
the single system deployment problem from first principles many years ago,
which many other deployment tools (Ansible, Puppet etc.) still cannot match
(the original paper [^3] explains this much better than me - the only tool in the
same league I can find is GNU Guix which is also based on Nix's core
principles, but its ecosystem is not as mature compared to NixOS)

7. With NixOS I can easily spin up VMs in QEMU with the same configuration
without applying it on real hardware [^4], and optionally write e2e tests in
Python using their testing framework. Talos can also do this with Talos in
Docker but running k8s in Docker has some quirks (this was a while ago so those
quirks might have been fixed already)

8. I use NixOS everywhere else including my laptops, work Macbooks (via
nix-darwin), gaming PC, backup server, etc., so it's nice to have a single way
to manage all of my computes (and also install them via PXE boot with
[nixie](https://github.com/khuedoan/nixie) - WIP)

9. This is a personal thing but when I use NixOS I haven't felt the need to
distro hop again, unlike with other distros (ok I still feel that a tiny bit
when seeing Guix, but still)

[^1]: I've tried a few other alternatives but none have come close to
k8s so far (e.g. [Nomad with various drivers](https://github.com/khuedoan/tinycloud),
also it was relicensed to BSL)
[^2]: [The Purely Functional Software Deployment Model](https://edolstra.github.io/pubs/phd-thesis.pdf)
[^3]: NixOS A/B partition upgrade example https://github.com/applicative-systems/nixos-appliance-ota-update
[^4]: NixOS test VM in QEMU example https://github.com/khuedoan/nixos-setup/blob/master/Makefile#L12-L15
