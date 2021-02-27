---
title: Learn Vim the easy way
date: 2021-02-27T12:28:55+07:00
cover:
  image: TODO
tags:
  - vim
draft: true
---

One thing about learning Vim I don't see people talk about is it not that hard if you don't try hard. You don't have to remove your mouse, stop using the arrow key, use vanilla vim on the terminal on the first day! Although you will learn slower this way, it will reduce the chance that's you will give up.

My recommendation is just install the vim plugin in your current IDE, learn the basic things first, use the mouse if needed, and build up your skill graduately.

Below is the complete road map:

## Day 1

- Install Vim plugin in your current IDE or text editor:
  - Visual Studio Code
  - Sublime
  - IntelliJ
- Learn about `NORMAL` mode and `INSERT` mode.
- Learn about `b` `e` `w`, `d`, `y`, `p`
- Keep using the mouse in `INSERT` mode, occasonally get back to `NORMAL` mode and move between words.

## Day 2

If you're not giving up yet, go through the vim tutor, it will take about 30 minutes:

```sh
# Ubuntu/Debian
sudo apt install neovim
# Fedora/CentOS
sudo yum install neovim
# macOS
brew install neovim
```

```sh
# Vim
vimtutor
# NeoVim
nvim +Tutor
```

## Day 30

After get to the same level of efficientcy using only the keyboard, if you want more extensibility and better performance, you can switch entirely to vanila vim.

Acutally don't use Vim, use NeoVim instead, it has multiple advantages:

- Better default config
- Cursor shape changed based on mode by default, trust me it will help your experience tramendously
- Most new features comes to NeoVim first
- More plugins with Lua support
- Legacy code removed
- A little bit faster
