HashiCorp has [changed their license to a source-available one](https://www.hashicorp.com/blog/hashicorp-adopts-business-source-license).
If you care about free software or are worried about the future of these projects,
here are some alternatives.

## Drop-in replacements

These hard forks can be used with almost no changes in the rest of your setup.

- Terraform: [OpenTofu](https://opentofu.org), a Linux Foundation project.
- Vault: [OpenBao](https://openbao.org), a Linux Foundation project.

## Replacable with similar products

While there's no 1:1 replacement, these projects can be replaced with others that have similar features or more.
They require significant changes to your infrastructure, though.

- Nomad: [Kubernetes](https://kubernetes.io) is the de-facto standard for container orchestration.
- Consul: Opt for any service mesh compatible with Kubernetes,
  such as [Istio](https://istio.io) or [Linkerd](https://linkerd.io) (both are graduated CNCF projects).
- Boundary: [Teleport](https://goteleport.com) is a similar open-source product.

## Replacable by composing other tools

There's no clear replacement for these products, but you can compose other tools to achieve similar features.
Fortunately, these products focus more on local development and developer experience, so there's less risk of breaking production.

- Vagrant: If you only need Ubuntu VMs, [Multipass](https://multipass.run) should suffice.
  However, most local development setups should move to containers now.
- Packer: If you're using Kubernetes, it's less likely that you'll need to build VM images.
  However, if you do need to, there's usually a distro-specific tool available to build a custom image for that distro.
  Personally, I use NixOS with [nixos-generators](https://github.com/nix-community/nixos-generators).
- Waypoint: Any combination of CI/CD tools should help you achieve a similar result,
  but you will have to invest much more time to integrate them.
  Personally, I use [Tekton](https://tekton.dev) and [Argo CD](https://argo-cd.readthedocs.io).

## Conclusion

Rug pulling is not uncommon in the open-source space. I hope you're not too locked in with HashiCorp products.

To be clear, I don't have a problem with people wanting to make money from their hard and honest work.
However, I do have a problem with companies that pretend to care about open source, build a community,
drive adoption and contributions, and then tell the community "Nah, we've changed our minds and will
restrict what you can do with the code, including the contributions made by the community".

When evaluating a new open-source project, I usually check if they have a Contributor License Agreement (CLA).
That's a red flag that the company is leaving the door open to rug pull you.
Sometimes I still decide to use them, but I always check if there's an alternative to jump to if the ship sinks.
