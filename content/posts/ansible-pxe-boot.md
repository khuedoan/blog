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

So you've been installing Linux manually on you hardware nodes manually: plugin the USB, go through the installer, and repeat (in my case I have 4 of them), this will take about 20-30 minutes per node

Or you've been doing it semi automatically, add a `preseed` or `kickstart` config to the USB installer, plug it in to your server, power it on and wait for the installation process, then manually change the config for each of them (hostname, IP...), this will take about 10 minutes per node

Imagine you can bring all of your server up with a single command from an empty hard drive in parallel and it's done when you finished your cup of coffee (5 minutes), without even touch the server's power button to turn it on :wink:
