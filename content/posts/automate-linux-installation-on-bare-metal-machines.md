---
title: "How to automate Linux installation on bare-metal machines with Docker and Ansible"
date: 2021-06-05T11:55:21+07:00
cover:
  image: https://ia601405.us.archive.org/29/items/khuedoan-blog-images/automate-linux-installation-on-bare-metal-machines-cover.jpg
tags:
  - ansible
  - automation
  - docker
  - homelab
draft: true
---

If you're like me, you have a homelab with multiple bare-metal machines.

Maybe you've been installing Linux manually on you hardware nodes manually: plugin the USB, keyboard, mouse and monitor, go through the installer, and repeat (in my case I have 4 of them), this will take about 15-20 minutes per machine.

Imagine you just need to run a single `make` command, all the servers gets turned on and Linux gets installed after 5 minutes, without needing you to even touch the power button.

We will use Ansible to create a stateless Docker-based PXE server, turn the nodes on with wake on LAN, boot them through the network, and install Linux using unattended install config file, and get host sepecific settings based on the MAC address.

## Prerequisite

```
┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│ server 0 │ │ server 1 │ │ server 2 │ │ server 3 │
└─────▲────┘ └─────▲────┘ └─────▲────┘ └─────▲────┘
      │            │            │            │
      └────────────┴─────┬──────┴────────────┘
                         │
                    ┌────┴───┐
                    │ swtich │
                    └────▲───┘
                         │
                         │
          ┌──────────────┴─────────────┐
          │           laptop           │
          │ ┌──────┐ ┌──────┐ ┌──────┐ │
          │ │ DHCP │ │ TFTP │ │ HTTP │ │
          │ └──────┘ └──────┘ └──────┘ │
          │      (Docker containers)   │
          └────────────────────────────┘
```

- BIOS settings:
  - Wake on LAN enabled
  - PXE boot enabled
  - Network boot set as default when wake from the network
  - Ethernet (doesn't work on Wifi)
- Control node (laptop, PC)
  - `docker`
