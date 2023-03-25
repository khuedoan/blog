---
title: Introduction to kmonad - a software alternative to QMK firmware
date: 2021-09-13T12:36:53+07:00
coverImage: TODO
tags:
  - kmonad
  - keyboard
draft: true
---

QMK (Quantum Mechanical Keyboard) is an awesome keyboard firmware, but not all keyboards are supported.
In my case, none of my keyboards is supported by QMK

## Installation

Refer to the [Installation document](https://github.com/kmonad/kmonad/blob/master/doc/installation.md#arch-linux) for the detailed instruction, bellow are some quick references.

### Linux

```sh
paru -S kmonad-bin
```

### macOS


Tested on Catalina (older/newer versions might be different, please check the official instruction)

**Note**: Uninstall Karabiner first, it will conflict with kmonad.

```sh
# Install build tool
brew install stack

# Clone kmonad source
git clone --recursive https://github.com/kmonad/kmonad.git
cd kmonad

# Install dext
sudo installer -package c_src/mac/Karabiner-DriverKit-VirtualHIDDevice/dist/Karabiner-DriverKit-VirtualHIDDevice-1.15.0.pkg -target LocalSystem
/Applications/.Karabiner-VirtualHIDDevice-Manager.app/Contents/MacOS/Karabiner-VirtualHIDDevice-Manager activate

# Compile kmonad
stack build --flag kmonad:dext --extra-include-dirs=c_src/mac/Karabiner-DriverKit-VirtualHIDDevice/include/pqrs/karabiner/driverkit:c_src/mac/Karabiner-DriverKit-VirtualHIDDevice/src/Client/vendor/include
```

### Windows

TODO

## Remap my keyboard
