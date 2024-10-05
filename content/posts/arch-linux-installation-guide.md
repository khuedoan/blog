But it can be confusing for new users because of how much information it provides.
In this tutorial, I'll show you how I install Arch Linux, without showing you too many choices.

## Prerequisites

- Your desktop/laptop
- A USB key
- Some basic knowledge of Linux

## The base system

### Verify the boot mode

Make sure you're booting in EFI mode, check if the directory exists:

`ls /sys/firmware/efi/efivars`

### Connect to the internet

#### Ethernet

You don't need to do anything.

#### Wifi

Get your device name:

`iwctl station list`

Scan for networks:

`iwctl station wlan0 get-networks`

Connect to your network:

`iwctl station wlan0 connect MYWIFI`

### Update the system clock

Ensure the system clock is accurate:

`timedatectl set-ntp true`

Check the service status:

`timedatectl status`

### Partition the disks

Identify disks:

`lsblk`

Disks are assigned to a *block device* such as `/dev/nvme0n1`.

Clean the entire disk (**do not** do this if you want to keep your data):

* `# gdisk /dev/nvme0n1`
* `x` for extra functionality
* `z` to *zap* (destroy) GPT data structures and exit
* `y` to proceed
* `y` to blank out MBR

Create boot partition and root partition:

* `# cfdisk /dev/nvme0n1`
* Select `gpt`
* Hit `[   New   ]` to create a new patition
* Give the boot partition `1G` and let the rest for the root partition
* Select the boot partition and hit `[   Type   ]` to choose `EFI System`
* Hit `[   Write   ]` then type `yes` to save, then hit `[   Quit   ]`

### Format the partitions

Format the boot partition to FAT32:

`mkfs.fat -F32 /dev/nvme0n1p1`

Format the root partition to ext4:

`mkfs.ext4 /dev/nvme0n1p2`

### Mount the file systems

Mount root partition first:

`mount /dev/nvme0n1p2 /mnt`

Then create mount point for boot partition and mount it accordingly:

`mkdir /mnt/boot`

`mount /dev/nvme0n1p1 /mnt/boot`

### Install the base and base-devel packages

Use the **pacstrap** script:

`pacstrap /mnt base linux linux-firmware base-devel`

### Generate an fstab file

Use `-U` or `-L` to define by UUID or labels:

`genfstab -U /mnt >> /mnt/etc/fstab`

### Chroot

Change root to the new system:

`arch-chroot /mnt`

### Install some essential packages

Microcode:

`pacman -S intel-ucode` (or `amd-ucode`)

Network manager:

`pacman -S networkmanager`

Your text editor of choice:

`pacman -S neovim` (or `nano` or `micro`)

And some packages for my personal config:

`pacman -S ansible git zsh`

### Create swap file

As an alternative to creating an entire swap partition, a swap file offers the ability to vary its size on-the-fly, and is more easily removed altogether.

Create a 32GiB (adjust the number depending on your RAM, I recommend a number equal to the amount of RAM) swap file:

`fallocate -l 32GiB /swapfile`

Set the right permissions:

`chmod 600 /swapfile`

format it to swap:

`mkswap /swapfile`

Activate the swap file:

`swapon /swapfile`

Edit fstab at `/etc/fstab` to add an entry for the swap file:

`nvim /etc/fstab`

```
/swapfile none swap defaults 0 0
```

### Configure time zone

Set your time zone by region:

`ln -sf /usr/share/zoneinfo/Asia/Ho_Chi_Minh /etc/localtime`

Generate `/etc/adjtime`:

`hwclock --systohc`

### Configure locale

Uncomment `en_US.UTF-8 UTF-8` in `/etc/locale.gen` (or just overwrite the file like below), then generate locale:

`echo 'en_US.UTF-8 UTF-8' > /etc/locale.gen`

`locale-gen`

Set LANG variable in `/etc/locale.conf`:

`echo 'LANG=en_US.UTF-8' > /etc/locale.conf`

### Change host name

Create hostname file at `/etc/hostname` contain the host name, for example:

`echo 'Precision' > /etc/hostname`

### Set your root password

`passwd`

Enter your password then confirm it.

### Install boot loader

Install `systemd-boot` to the `/boot` partition:

`bootctl --path=/boot install`

Edit `systemd-boot` options:

`nvim /boot/loader/loader.conf`

```
default arch
timeout 0
```

Add Arch boot entry:

`nvim /boot/loader/entries/arch.conf`

```
title   Arch Linux
linux   /vmlinuz-linux
initrd  /intel-ucode.img
initrd  /initramfs-linux.img
options root=/dev/nvme0n1p2 rw
```

### Enable network services

`systemctl enable NetworkManager`

### Add new user

Add a new user named `myname`:

`useradd -m -G wheel -s /bin/zsh -c "My Name" myname`

Protect the newly created user `myname` with a password:

`passwd myname`

Establish `nvim` as the **visudo** editor:

`EDITOR=nvim visudo`

Then uncomment `%wheel ALL=(ALL) ALL` to allow members of group `wheel` sudo access.

### Reboot to the new system

Exit the chroot environment by typing:

`exit`

Restart the machine:

`reboot`

### Login

Login with your user account after the machine has rebooted. Use `nmtui` to connect to the Internet if you're using wifi.

## Graphical user interface

You'll have a lot of choices when it comes to Linux user interface, but for the sake of tutorial, I'll choose [GNOME](https://wiki.archlinux.org/title/GNOME) because it's the simplest one to install and use.

`sudo pacman -S gnome`

`sudo systemctl enable --now gdm`

Personally I don't like GNOME, here's some of my recommendation:

- [KDE](https://wiki.archlinux.org/title/KDE): Fast, looks nice by default, tons of feature
- Build your own desktop environment with a window manager:
  - [bspwm](https://wiki.archlinux.org/title/Bspwm): Minimal and fast
  - [dwm](https://dwm.suckless.org): hackable, designed to add your own feature to the code base, written in C

Checkout the [r/unixporn](https://reddit.com/r/unixporn/) subreddit for much more eye candy screenshot of Linux.
