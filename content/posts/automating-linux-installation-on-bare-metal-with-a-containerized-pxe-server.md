If you're like me, you have a homelab with multiple bare-metal machines. Maybe
you've been installing Linux manually on your servers:

- Download the ISO of your favorite server distro, and `dd` it to a USB
- Plug in the live USB, keyboard, mouse, and monitor
- Boot into the live USB and run the installer
- Repeat for each server (I have 4, so this takes 15-20 minutes per machine)

Now, imagine starting from empty hard drives on your servers - you run a single
`make` command from your laptop, and all the servers power on automatically (no
need to touch the power button). In just 5 minutes, Linux is installed on all
of them.

Enough talk - let's see a demo!

<div style="position:relative; padding-bottom:56.25%; height:0; overflow:hidden; max-width:100%;">
    <iframe
        src="https://www.youtube-nocookie.com/embed/y-d7btNNAT8"
        title="YouTube video player"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
        referrerpolicy="strict-origin-when-cross-origin"
        allowfullscreen
        style="position:absolute; top:0; left:0; width:100%; height:100%;"
    ></iframe>
</div>

## How it works

We'll set up a **stateless**
[PXE](https://en.wikipedia.org/wiki/Preboot_Execution_Environment) server using
Docker containers on our laptop. A stateless PXE server is ideal for a homelab
because a stateful setup requires manual provisioning of the initial PXE
server, or migrating its state to a persistent server after bootstrapping the
machines. With a stateless PXE server, you can just nuke it once the
installation is complete.

The process includes:

1. **Powering on nodes** using [Wake-on-LAN](https://en.wikipedia.org/wiki/Wake-on-LAN).
2. **Network booting** via the PXE boot process.
3. **Automating Linux installation** using a preconfigured unattended
   installation file, with host-specific configurations determined by each
   machine's MAC address.
4. **Decommissioning the PXE environment** after successful installations.

The PXE server consists of three core services running in Docker containers:

- **DHCP server:** Assigns IP addresses and points clients to the TFTP server.
- **TFTP server:** Delivers boot files, including the PXE bootloader and kernel images.
- **HTTP server:** Hosts installation files and configuration scripts.

The laptop connects to the same network as the bare-metal machines using Docker's host network mode:

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

The laptop will send a magic packet to wake the servers up, then the servers start booting in network mode.

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

If you're already familiar with normal USB installation, here's a comparison with PXE boot installation:

| Normal USB installation | PXE boot installation |
| --- | --- |
| Boot menu entry specifies the boot partition location | DHCP server provides the TFTP server's IP |
| Boot partition (e.g., `/boot`) contains the bootloader and config files | TFTP server serves the bootloader and config files |
| User enters options in the installer manually | Automated boot instructions (kickstart, preseed, ignition config, etc.) are downloaded from the HTTP server |
| Installer copies packages, binaries, and config files from the USB to the target disk | Packages, binaries, and config files are downloaded from the HTTP server |

The automated install config file depends on the distro you're using:

- RHEL, CentOS, Rocky Linux, Fedora:
  [kickstart](https://docs.fedoraproject.org/en-US/fedora/rawhide/install-guide/advanced/Kickstart_Installations/)
- Debian: [preseed](https://wiki.debian.org/DebianInstaller/Preseed)
- Ubuntu: [autoinstall](https://ubuntu.com/server/docs/install/autoinstall)
- CoreOS: [ignition](https://coreos.github.io/ignition)
- etc.

In this post, I'll use Fedora with kickstart because that's what I'm running at
the moment, but the same apply for other distro, just change the boot parameter
and the installation config file.

## Create a basic PXE server

### Prerequisite

BIOS settings on bare-metal machines:

- Wake on LAN enabled
- PXE boot enabled
- Network boot set as the default when waking from the network
- Ethernet (Wi-Fi doesn't work)
- Note their MAC addresses

Temporary control node (laptop, PC):
- `docker` with host network support
- Connected to the same network as the machines

### Project structure

We will use Docker Compose to create the PXE server, which includes three small
servers: DHCP, TFTP, and HTTP. Create the following project structure:

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

### Generate the configuration dynamically with Ansible

I'm using Ansible to generate to config files from
[templates](https://github.com/khuedoan/homelab/tree/master/metal/roles/pxe_server/templates),
you can checkout my Ansible role for that
[here](https://github.com/khuedoan/homelab/tree/master/metal/roles/pxe_server).

Initially, I wanted to include many code examples in this post, but I realized
it would be too long and hard to read. So it's easier to go directly to the
repository and check the code there. :)

### Start the machines

Now, we can start the machines. I'm using Wake-on-LAN to power them on, and my
BIOS settings are configured to boot from the network first if the machine is
woken up by Wake-on-LAN:

```yaml
- name: Send Wake-on-LAN magic packets
  community.general.wakeonlan:
    mac: "{{ mac }}"
  delegate_to: localhost

- name: Wait for the machines to come online
  ansible.builtin.wait_for_connection:
    timeout: 600
```

You can find the full playbook [here](https://github.com/khuedoan/homelab/blob/master/metal)

## Conclusion

I can't count how many hours it has saved me. I hope this post has inspired you
to set up your own PXE server. If you have any questions, feel free to ask in
my [homelab project](https://github.com/khuedoan/homelab).
