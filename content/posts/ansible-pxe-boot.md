---
title: "Automate OS installation on bare metal servers with PXE boot and Ansible"
date: 2021-02-25T11:55:21+07:00
tags:
  - ansible
  - automation
  - devops
  - pxe
  - wol
draft: true
---

If you're like me, you have a homelab with multiple bare metal servers.

Maybe you've been installing Linux manually on you hardware nodes manually: plugin the USB, go through the installer, and repeat (in my case I have 4 of them), this will take about 20-30 minutes per node

Maybe you've been doing it semi automatically, add a `preseed` or `kickstart` config to the USB installer, plug it in to your server, power it on and wait for the installation process, then manually change the config for each of them (hostname, IP...), this will take about 10 minutes per node

Imagine you can get a cup of coffee, run a single command, all the servers gets turned on and Linux gets installed with host specific settings in parallel and everything is done before you even finished your :coffee:, doesn't matter if you have 1 or 10 nodes.

We will use Ansible to create a stateless Docker-based PXE server, turn the nodes on with wake on LAN, boot them through the network, and install Linux using `kickstart` or `preseed`, and get host sepecific settings based on the MAC address

## Prerequisite

- Basic knowledge about Ansible, Docker and networking
- BIOS settings:
  - Wake on LAN enabled
  - PXE boot enabled
  - Network boot set as default when wake from the network
  - Ethernet interface
- Control node (laptop, PC)
  - `docker`
  - `python`
  - `make`

## Makefile

```make
.POSIX:

default: init run

init:
    python3 -m venv .venv \
        && . .venv/bin/activate \
        && pip3 install --upgrade pip \
        && pip3 install -r requirements.txt

run:
    . .venv/bin/activate \
        && ansible-playbook --inventory hosts.ini playbook.yml
```
