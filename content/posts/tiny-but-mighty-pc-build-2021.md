---
title: Tiny but Mighty Pc Build 2021
date: 2021-10-23T20:47:06+07:00
cover:
  image: TODO
tags:
  - TODO
draft: true
---

## Parts

## BIOS upgrade without CPU

source https://us.msi.com/support/technical_details/MB_BIOS_Update

### Get the latest BIOS version

Download latest BIOS to support 5600X (careful not use beta)

https://www.msi.com/Motherboard/support/B450I-GAMING-PLUS-AC#down-bios

unzip the downloaded file:

`unzip 7A40vAC.zip`

### Prepare my USB

Plug it in:

`lsblk`

```
NAME   MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
sda      8:0    0 119.2G  0 disk
├─sda1   8:1    0   512M  0 part /boot
└─sda2   8:2    0 118.7G  0 part /var/lib/docker/btrfs
                                 /
sdb      8:16   1  14.5G  0 disk
├─sdb1   8:17   1   747M  0 part
└─sdb2   8:18   1    84M  0 part
```

Wipe the disk:

`gdisk /dev/sdb # TODO`

Create new partition:

`cfdisk /dev/sdb`

Format new partition as FAT32:

`sudo mkfs.fat -F32 /dev/sdb1`

Create new directory and mount the USB:

```sh
mkdir ~/usb
sudo mount /dev/sdb1 ~/usb
```

Then copy the BIOS files to the USB:

```sh
sudo cp -r ~/Downloads/7A40vAC ~/usb/
```

`tree usb/7A40vAC/`

```
usb/7A40vAC/
├── 7A40vAx.txt
└── E7A40AMS.AC0

0 directories, 2 files
```

Unmount the USB:

```sh
sudo umount ~/usb
```

Then unplug it and plug in to this USB port:

TODO


