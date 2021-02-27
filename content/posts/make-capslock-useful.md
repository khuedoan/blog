---
title: Turn the useless Capslock key into a magic key
date: 2021-02-27T10:32:49+07:00
cover:
  image: TODO
tags:
  - keyboard
  - vim
draft: true
---

Have you ever wonder why such a useless key sitting in a expensive realestate spot of your keyboard, right on the home row next to your pinky, and you have to stretch your finger to reach other regularly used keys?

TODO PIC here: control, escape, backspace

If you're already typing "THEN HOW WOULD I TYPE A LONG CAPITALIZED SENTENCE LIKE THIS???" in the comment section, then I guess you like to shout at people, or you work with SQL alot. In that case you can still preserve the Capsock functionality, or you can toggle Capslock when pressing both Shift.

## What that key should do instead

You can make it much more useful if you map it to a regularly used modifier key, and another regularly used normal key when tap. Depending on your usecase, here is some suggestions:

### Control when hold, Capslock when tap

This is the easiest to get used to and will fits most people

TODO GIF: control Capslock

### Control when hold, Backspace when tap

sligthly upgraded version of the previous one, which help you don't have to stretch your pinky to the backspace:

TODO GIF: control backspace

### Control when hold, Escape when tap

Extreamily useful for vim users. You may already map `jk` to exit `NORMAL` mode, but it takes 2 keystrokes, and will not work when you SSH into a remote server.

TODO GIF: control esc

## How

In these example I will use Ctrl and Escape, but you can change it any combination of keys.

### Linux/BSD

Install `xcape`:

```sh
sudo apt install xcape
```

Just put it in your startup script:

```sh
xcape
```

### macOS

The most used modifer key on macOS - the Command key - is already in good position (next to your thumb), But this will still benefit you if your're using vim or emacs, which requires Ctrl or Option (Meta/Alt) regularly.

```sh
brew install --cask karabiner-elements
```

Complex modification

```json
{
  "description": "Post Esc if Caps is tapped, Control if held.",
  "manipulators": [
    {
      "from": {
        "key_code": "left_control",
        "modifiers": {
          "optional": [
            "any"
          ]
        }
      },
      "to": [
        {
          "key_code": "left_control",
          "lazy": true
        }
      ],
      "to_if_alone": [
        {
          "key_code": "escape"
        }
      ],
      "type": "basic"
    }
  ]
}
```

### Windows

Don't use Windows, it's garbage. Install Linux!

Just kidding, you can download [dual-key-remap](https://github.com/ililim/dual-key-remap) and make it autostart.

Put this in your `config.txt`

```cfg
remap_key=CAPSLOCK
when_alone=ESCAPE
with_other=CTRL
```
