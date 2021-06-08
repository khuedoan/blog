---
title: "Automated Linux installation on bare metal using a containerized PXE server"
summary: Install Linux (CentOS, Fedora, Debian, Ubuntu...) on all of your bare-metal machines in under 5 minutes without even touching them
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

Enough talk, time for a demo:

TODO demo video here

## How it works

We will create a stateless PXE server in Docker containers on our laptop, turn the nodes on with wake on LAN, boot them through the network, and install Linux using unattended install config file, and get host sepecific settings based on the MAC address. We can shut down the PXE server after everything is installed.

My laptop has 3 small servers inside Docker containers:

- DHCP server
- TFTP server
- HTTP server

It connected to the same network with my bare-metal machines using host network:

```txt
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
             │ ┌───┴──┐ ┌───┴──┐ ┌───┴──┐ │
             │ │ DHCP │ │ TFTP │ │ HTTP │ │
             │ └──────┘ └──────┘ └──────┘ │
             │  laptop (docker-compose)   │
             └────────────────────────────┘
```

The laptop will send a magic packet to wake the servers up (Wake-on-LAN), then the servers start booting in network mode.

You can map the installation process from the network to the installation process from a USB:

- Boot menu entry provides the boot partition location: DHCP server provides the TFTP server's IP
- Boot partition (`/boot`) contains the boot loader and the boot config: TFTP server
- User enter the options in the installer: automated boot instruction (kickstart, preseed, ignition config...) downloaded from the HTTP server
- The installer install the required packages, binary, config files... from the disk: download those stuff from the HTTP server

The diagram below show the network boot process

```txt
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

The automated install config file depends on the distro you're using:

- RHEL, CentOS, Rocky Linux, Fedora: [kickstart](https://docs.fedoraproject.org/en-US/fedora/rawhide/install-guide/advanced/Kickstart_Installations/)
- Debian: [preseed](https://wiki.debian.org/DebianInstaller/Preseed)
- Ubuntu: [autoinstall](https://ubuntu.com/server/docs/install/autoinstall)
- CoreOS: [ignition](https://coreos.github.io/ignition/)

In this tutorial, I'll use Fedora with kickstart because that's what I'm running at the moment (and it's really easy to get started with kickstart too, you'll see why below), but the same apply for other distro, just change the boot parameter and the installation config file.

## Create a basic PXE server

### Prerequisite

- BIOS settings on bare-metal machines:
  - Wake on LAN enabled
  - PXE boot enabled
  - Network boot set as default when wake from the network
  - Ethernet (doesn't work on Wifi)
  - Note their MAC address
- Control node (laptop, PC)
  - `docker` with host network support
  - Connected to the same network with the machines

### Project structure

We will use `docker-compose` to create the PXE server (which contains 3 small servers: DHCP, TFTP and HTTP).
Create a project as follow (just `touch` the empty files):

```
├── docker-compose.yml
├── images/
├── mnt/
├── dhcp/
│   ├── dhcpd.conf
│   └── Dockerfile
├── tftp/
│   ├── Dockerfile
│   └── tftpboot/
│       └── grub.cfg
└── http/
    ├── Dockerfile
    └── kickstart/
        └── fedora.ks
```

## Generate the configuration dynamically with Ansible

The basic PXE server we just created has a bunch of hard coded values.
I'm using Ansible to generate to config files from [templates](https://github.com/khuedoan/homelab/tree/master/metal/roles/pxe-boot/templates), you can checkout my Ansible role for that [here](https://github.com/khuedoan/homelab/tree/master/metal/roles/pxe-boot).
