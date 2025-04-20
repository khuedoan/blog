# Fixing a weird bug on the Intel 219-LM Ethernet controller

I recently set up a [ThinkCentre
M710q](https://www.lenovo.com/us/en/p/desktops/thinkcentre/m-series-tiny/thinkcentre-m710q/11tc1mt710q)
for my homelab staging environment. After plugging in the Ethernet cable, I
couldn’t get an internet connection to work under Linux, though it worked fine
on Windows. I had to go down a rabbit hole just to get Ethernet working, so I’m
sharing my experience in hopes it helps someone else in the future.

I started by confirming that the Ethernet adapter was recognized by the system:

```sh
lspci -v | grep -A 10 Ethernet
```

The system detected an Intel Ethernet controller, so the hardware was
recognized. The driver was also loaded:

```sh
lsmod | grep e1000e
```

I then checked the system logs for errors related to the Ethernet controller:

```sh
dmesg | grep e1000e
```

There were no immediate issues, which was puzzling. I decided to try connecting
via Wi-Fi:

```sh
iwctl
station list
station wlan0 get-networks
station wlan0 connect MYWIFI
```

After some research, I found [this answer on Super
User](https://superuser.com/questions/1104537/how-to-repair-the-checksum-of-the-non-volatile-memory-nvm-of-intel-ethernet-co/1106641#1106641).
, which suggested updating the Intel Ethernet controller firmware. Here’s how I
did it:

1. Download the firmware utility:

```sh
curl -O https://downloadmirror.intel.com/772083/Preboot.tar.gz
```

2. Extract the contents:

```sh
tar xvf Preboot.tar.gz
```

3. Navigate to the BootUtil directory:

```sh
cd APPS/BootUtil/Linux_x64
```

4. Make the utility executable and reset the controller:

```sh
chmod +x bootutil64e
./bootutil64e -NIC 1 -DEFAULTCONFIG
```

This command reset the NIC to its default configuration, resolving the issue.

After running these steps, my Ethernet connection was restored, and the
ThinkCentre M710q was successfully connected to the network.
