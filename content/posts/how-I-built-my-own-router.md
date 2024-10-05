Download OPNsense: https://opnsense.org/download

Get a USB key and write the image:

```sh
bunzip2 OPNsense-22.7-OpenSSL-vga-amd64.img.bz2
dd if=OPNsense-22.7-OpenSSL-vga-amd64.img of=/dev/sdX bs=16k
```
