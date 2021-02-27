---
title: Make the useless Capslock key useful
date: 2021-02-27T10:32:49+07:00
cover:
  image: TODO
tags:
  - keyboard
  - vim
draft: true
---

Have you ever wonder why such a useless key sitting in a expensive realestate spot of your keyboard? You can make it much more useful if you map it to a regularly used modifier key like Control or Command, and regularly used key when tap, like Escape or Backspace (or maybe the Capslock key itself)

- Control when hold, Capslock when tap (useful for someone who use the Capslock key regularly)
- Control when hold, Backspace when tap
- Control when hold, Escape when tap (very useful for vim users, this is what I'm using)

## So what should it do then?

How we normally type:

TODO PIC here: control, escape, backspace

TODO GIF: control esc

TODO GIF: control backspace

TODO GIF: control Capslock

## How do I make it like that?

### Linux/BSD

`xcape`

Just put it in your startup script:

```sh
xcape
```

### macOS

It's a little more complex

`karabiner`

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

You should install Linux or buy a Mac tbh, Windows is garbage.

Just kidding, you can download [dual-key-remap](https://github.com/ililim/dual-key-remap) and make it autostart.
