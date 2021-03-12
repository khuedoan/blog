---
title: How to ease the pain when learning Vim
summary: Vim is hard to learn, that's a fact. But it doesn't have to be (that) painful. This article will cover a road map and how to not get frustrated when learning vim
date: 2021-02-27T12:28:55+07:00
cover:
  image: TODO
tags:
  - vim
draft: true
---

One thing about learning Vim I don't see people talk about is it not that hard if you don't try hard. You don't have to remove your mouse, stop using the arrow key, use vanilla vim on the terminal on the first day! Although you will learn slower this way, it will reduce the chance that's you will give up.

My recommendation is just install the vim plugin in your current IDE, learn the basic things first, use the mouse if needed, and build up your skill graduately.

Vim already has an excellent tutorial built in, I will not repeat that again.
This is just a learning path and to give you a glimps about what to expect.
The most important thing is not how fast you can learn it, but to not give up.

Below is the complete road map. You don't have to strictly follow this

## Day 1

You should probably do this on the weekend because the set up may takes some time.

Install Vim plugin in your current IDE or text editor (for the sake of simplicity I'll call all of them IDE).
Most of the popular one has a vim emulator plugin.
But in this article I will cover Visual Studio Code because it's the most popular.

You have 2 options:

  - Use the vim emulator plugin: this is easy to set up, the performance is not great but it's fine if you're not too sensitive about input latency.
  - The new neovim client: Use a real neovim server under the hood, great performance but the stability is not there yet because it's quite new.

Regardless of which plugin or IDE you use, you should install the commandline version of vim to go through the tutorial.
Learn the theory on real vim and practice with your with day to day coding work on your current IDE

Actually don't use vim, use Neovim instead.
Neovim is basically vim without tons of technical debt and legacy stuff.
It also more beginner friendly with better default config and absolute killer feature that's enabled by default but I don't see anyone talk about: change cursor shape base on mode!

Here's a demo:

| vim | neovim |
|-----|--------|
| GIF | GIF |

As you can see 

Learn about `NORMAL` mode and `INSERT` mode. Keep using the mouse just like you usually do, the point of the first day is just to get the software set up and get used to 2 different main mode of vim.

## Day 2

If you're not giving up yet (I know you're not, you can do it!), go through the vim tutor, it will take about 30 minutes.

```sh
nvim +Tutor
```

Or type `:Tutor` from inside vim.

Go through section `1.x`, you don't have to remember all that, just the following is enough:

- Know how to save and quit
- Know how to delete things with `x` and `d`
- Move between words with `b` `e` `w`

That's it! Try to use them when you do your actual work :smile:

## Day 3 to 8

Go through one chapter a day.

## Day 8 to 30

Keep using what you've learn.
Use the mouse if needed, but every time you use the mouse, think about how can you do that with the keyboard.
Once you've feeling like you're doing something slower than what you did with a mouse, search on how to improve it.

## Day 31 on ward

After a month of learning vim, you got to the same level of efficiency just like using the mouse with only the keyboard.

If you want more extensibility and much better performance, you can try vim on the terminal to see if it worth it to you.
My recommendation is to using a framework like spacevim when starting out.
After you know what you want, start building your `init.lua` from scratch (or `init.vim` if you're using Neovim version below 0.5)
