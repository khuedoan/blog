---
title: Backup Your Otp
date: 2023-05-13T14:15:45+07:00
cover:
  image: TODO
tags:
  - TODO
draft: true
---

My setup to have a secure online experience, while preventing blocking myself out of my own accounts

## General rule of thumb

- Strong passwords in password manager
- OTP
- Yubikey if possible
- Backups !!!

create an FAT32 partition on your USB (4GB) to ensure compatibility across many systems.

label BACKUP (lowercase labels might not work properly on some systems)

```
                                     Disk: /dev/sda
                  Size: 57.3 GiB, 61530439680 bytes, 120176640 sectors
              Label: gpt, identifier: XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX

    Device              Start          End      Sectors     Size Type
    /dev/sda1            2048      8390655      8388608       4G Microsoft basic data
>>  /dev/sda2         8390656    120174591    111783936    53.3G Microsoft basic data


 ┌────────────────────────────────────────────────────────────────────────────────────┐
 │Partition UUID: XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX                                │
 │Partition type: Microsoft basic data (XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX)         │
 └────────────────────────────────────────────────────────────────────────────────────┘
   [ Delete ]  [ Resize ]  [  Quit  ]  [  Type  ]  [  Help  ]  [  Write ]  [  Dump  ]
```

1:2:3

OTP

- local password store
- usb key
- remote private git repository, only acceessible with SSH

encrypted with GPG, so


GPG

- local gpg store
- exported encrypted armored private key in usb key
- exported encrypted armored private key in password manager

password manager

- local vault
- remote vault from bitwarden
- exported encrypted vault JSON file in USB

encryption passwords

- bitwarden master password: brain
- gpg password: brain + bitwarden
- ssh key password: brain + something

last ditch effort:

- yubikey

