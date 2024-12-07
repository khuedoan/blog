After fine-tuning my ThinkPad Z13 laptop running Linux, I've significantly
reduced power consumption, especially with light usage. Below is a breakdown of
the setup and results, which may help others optimize their systems for better
battery life.

## Test baseline

Laptop: ThinkPad Z13 Gen 1 (AMD Ryzen 7 PRO 6850U, 16GB RAM, 512GB SSD)

- Brightness: 25% (`light -S 25`)
- Speaker: 50%
- Wi-Fi: On
- Bluetooth: Off (`rfkill block 1`)
- Window manager: Sway (Wayland)
- Browser: Ungoogled Chromium with VAAPI and native Wayland, no extensions

## CPU and GPU tuning

1. Use kernel >= v6.5 for some new hardware supports, with `amd_pstate` active
   for better CPU power management, providing significant battery savings.
2. Use TLP >= v1.6 for some new hardware supports: TLP is essential for
   laptops, optimizing power usage with minimal intervention. Adjust EPP
   (Energy Performance Policy) to power-saving mode.
3. CPU governor: set the governor to `powersave` by default, reducing power
   consumption when idle.
4. AMD GPU power mode: set to `low`, optimizing power usage for the integrated
   GPU. Note that this has caused some graphical glitches on my machine in the
   past, but I don't experience them anymore. YMMV.
5. Undervolting: helps reduce power draw without sacrificing performance.

Some example configuration in NixOS:

```nix
{
  services = {
    tlp = {
      enable = true;
      settings = {
        CPU_DRIVER_OPMODE_ON_BAT = "active";
        CPU_ENERGY_PERF_POLICY_ON_BAT = "power";
        CPU_SCALING_GOVERNOR_ON_BAT = "powersave";
        RADEON_DPM_PERF_LEVEL_ON_BAT = "low";
        PLATFORM_PROFILE_ON_BAT = "low-power";
        DEVICES_TO_DISABLE_ON_BAT_NOT_IN_USE = "bluetooth wifi wwan";
      };
    };
  };
}
```

I also have a small script to verify these settings, you can find it
[here](https://github.com/khuedoan/nixos-setup/tree/master/scripts/check_laptop_power.sh)

## Some gotchas

- Docker network interfaces may cause high power consumption. I disabled Docker
on boot and set it to start on demand by socket activation:

```nix
{
  virtualisation = {
    docker = {
      enable = true;
      enableOnBoot = false;
    };
  };
}
```

- Firefox consumes more power than Chromium for some reason. I still haven't
  figured out why, so I stick with Chromium for now.
- Sway fractional scaling: avoid fractional scaling for now to reduce the
  graphical load.

## Test Results

- Idle power usage: Less than 3W
- Light web browsing (up to 10 tabs, including Reddit, Hacker News, GitHub, etc.): 3-4W
- 1080p AV1 YouTube (30fps): 5W ([example video](https://www.youtube.com/watch?v=dQw4w9WgXcQ))
- 4K AV1 YouTube: 7.5W
- Closed lid overnight: lost ~1% of battery, and `powertop` shows ~0.2W
  consumption after waking up.

With these tweaks, my battery consumption dropped from 10-12W to 2-5W. Although
the machine feels a bit slower, I don't mind the slight performance hit for the
significant battery life improvement, since I mostly use my laptop for light
tasks.
