---
title: "How to automate Linux installation on bare-metal machines with Docker and Ansible"
summary: Install OS (like CentOS, Fedora, Debian, Ubuntu...) on all of your bare-metal machines in under 5 minutes
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

Maybe you've been installing Linux manually on you servers:

- Plugin the live USB, keyboard, mouse and monitor
- Boot into the live USB and go through the installer
- Repeat (in my case I have 4 of them), this will take about 15-20 minutes per machine

Imagine from empty hard drive on your servers, you just need to run a single `make` command on your laptop, all the servers gets turned on without needing you to even touch the power button, and Linux gets installed on all of them after 5 minutes.

## How it works

We will create a stateless PXE server in Docker containers on our laptop, turn the nodes on with wake on LAN, boot them through the network, and install Linux using unattended install config file, and get host sepecific settings based on the MAC address.

My laptop has 3 small servers inside Docker containers:

- DHCP server
- TFTP server
- HTTP server

It connected to the same network with my bare-metal machines using host network:

```
┌───────────┐ ┌───────────┐ ┌───────────┐       ┌───────────┐
│ machine 0 │ │ machine 1 │ │ machine 2 │  ...  │ machine N │
└─────┬─────┘ └─────┬─────┘ └─────┬─────┘       └─────┬─────┘
      │             │             │                   │
      │             │             │                   │
      └────────────┬┴───────┬─────┴──┬────────────────┘
                   │        │        │
                   │        │        │
                   │        │        │
                   │        │        │
             ┌─────┼────────┼────────┼────┐
             │     │        │        │    │
             │ ┌───┴──┐ ┌───┴──┐ ┌───┴──┐ │
             │ │ DHCP │ │ TFTP │ │ HTTP │ │
             │ └──────┘ └──────┘ └──────┘ │
             │                            │
             │  laptop                    │
             └────────────────────────────┘
```

The laptop will send a magic packet to wake the servers up (Wake-on-LAN), then the servers start booting in network mode.

```
┌───────────┐               DHCP request                 ┌────────┐
│ machine N │ ─────────────────────────────────────────► │        │
│           │    send next server IP and boot file name  │  DHCP  │
│           │ ◄───────────────────────────────────────── │        │
│           │                                            └────────┘
│           │                                      
│           │     request boot file and boot config      ┌────────┐
│           │ ─────────────────────────────────────────► │        │
│           │     send boot file and boot config         │  TFTP  │
│           │ ◄───────────────────────────────────────── │        │
│           │                                            └────────┘
│           │                                      
│           │  request automated install instruction     ┌────────┐
│           │ ─────────────────────────────────────────► │        │
│           │     send automated install instruction     │        │
│           │ ◄───────────────────────────────────────── │        │
│           │                                            │  HTTP  │
│           │     request packages, config files...      │        │
│           │ ─────────────────────────────────────────► │        │
│           │       send packages, config files...       │        │
│           │ ◄───────────────────────────────────────── │        │
│           │                                            └────────┘
│  reboot   │
└───────────┘
```

## Prerequisite

- BIOS settings:
  - Wake on LAN enabled
  - PXE boot enabled
  - Network boot set as default when wake from the network
  - Ethernet (doesn't work on Wifi)
- Control node (laptop, PC)
  - `docker` with host network support
  - Connected to the same network with the machines
