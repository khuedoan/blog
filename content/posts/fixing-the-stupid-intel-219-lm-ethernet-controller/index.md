---
title: Fixing the Stupid Intel 219-LM Ethernet Controller in Linux
date: 2023-03-07T10:57:41+07:00
tags:
  - TODO
draft: true
---

I ordered a ThinkCentre M710q for the staging environment in my homelab

Plugged in my Ethernet cable but no internet

Check `ip a` to see if it's connected and:


`lspci | grep Ethernet`
`lspci -v`

`lsmod | grep e1000e`

https://superuser.com/questions/1104537/how-to-repair-the-checksum-of-the-non-volatile-memory-nvm-of-intel-ethernet-co/1106641#1106641

`dmesg | grep e1000e`

`iwctl`

```
station list
station wlan0 get-networks
station wlan0 connect MYWIFI
```

Now I got internet working

Then I found

https://superuser.com/a/1170175

So basically:

```
curl -O https://downloadmirror.intel.com/772083/Preboot.tar.gz
tar xvf Preboot.tar.gz
cd APPS/BootUtil/Linux_x64
chmod +x bootutil64e
./bootutil64e -NIC 1 -DEFAULTCONFIG
```

Now it works!
