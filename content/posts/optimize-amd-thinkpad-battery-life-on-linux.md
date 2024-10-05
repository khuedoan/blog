Test environment:

- Brightness 25% (light -S 25)
- Speaker 50%
- Wifi on
- Bluetooth off (rfkill block 1)

- Sway (Wayland)
- Ungooogled Chromium with VAAPI enabled and native Wayland, no extension

From 10-12w to 2-5w.

- Use new kernel > 6.3 with amd_pstate active
- Use TLP (as you should on laptop)
- Use TLP > 1.6 to adjust epp to power
- Set governor to powersave by default
- Set AMD GPU power mode to low
- Undervolt
- Don't use Sway fractional scaling (yet)

Test case:

- New boot, idle: less than 3W
- Browsing the web with less than 10 tabs (reddit, hackernews, github): around 3-4W
- Watch AV1 1080p@30fps YouTube video in 1 tab: around 5W https://www.youtube.com/watch?v=dQw4w9WgXcQ
- Watch AV1 4K YouTube video in 1 tab: around 7.5W
- Closed lid overnight: lose ~1%, powertop shows ~0.2W after waking up
