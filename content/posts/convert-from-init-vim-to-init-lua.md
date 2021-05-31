---
title: How to convert Neovim config from init.vim to init.lua
summary: Only a few days left until Neovim 0.5 stable release, with the support of `init.lua` as default user config
date: 2021-03-12T11:59:12+07:00
cover:
  image: https://user-images.githubusercontent.com/27996771/110966944-69473f80-8388-11eb-9773-9594cbf662f4.png
tags:
  - vim
---

Only a few ~~days~~ months left until Neovim 0.5 stable release, with the support of `init.lua` as default user config.
Here's a quick guide (more like a cheat sheet) to help someone with no Lua experience like me to save a few hours :moon:

## Config path

`~/.config/nvim/init.vim` → `~/.config/nvim/init.lua` 

## Comment

```viml
" init.vim
" This is a comment
```

```lua
-- init.lua
-- This is a comment
```

## Options

```viml
" init.vim
set mouse=a
set tabstop=4
set number
set shortmess+=c
```

```lua
-- init.lua
vim.opt.mouse = 'a'
vim.opt.tabstop = 4
vim.opt.number = true
vim.opt.shortmess = vim.opt.shortmess + 'c'
```

## Global variables

Simmilar to the above

```viml
" init.vim
let g:mapleader = ' '
```

```lua
-- init.lua
vim.g.mapleader = ' '
```

For variables with namespace, Lua doesn't support `#` so you have to put them in `['']`:

```viml
" init.vim
let g:sneak#label = 1
```

```lua
-- init.lua
vim.g['sneak#label'] = 1
```

## Key mappings

```viml
" init.vim
nnoremap <C-s> :write<CR>
```

```lua
-- init.lua
--                       <mode>  <keys>    <actions>       <options>
vim.api.nvim_set_keymap( 'n',   '<C-s>', ':write<CR>', {noremap = true})
```

## Plugin manager

I've switched to [paq-nvim](https://github.com/savq/paq-nvim), written in Lua, very similar to [vim-plug](https://github.com/junegunn/vim-plug).

Install the plugin manager:

```lua
-- init.lua
-- Auto install if not exist
local install_path = fn.stdpath('data')..'/site/pack/paqs/opt/paq-nvim'

if fn.empty(fn.glob(install_path)) > 0 then
  cmd('!git clone --depth 1 https://github.com/savq/paq-nvim.git '..install_path)
end

-- Load the plugin manager
cmd 'packadd paq-nvim'

-- Set the short hand
local plug = require('paq-nvim').paq

-- Make paq manage it self
plug {'savq/paq-nvim', opt=true}
```

Install your plugins:

```lua
-- init.lua
plug 'joshdick/onedark.vim'
plug 'justinmk/vim-sneak'
plug 'neovim/nvim-lspconfig'
```

The commands is very similar too:

| vim-plug       | paq-nvim      |
|----------------|---------------|
| `:PlugInstall` | `:PaqInstall` |
| `:PlugClean`   | `:PaqClean`   |

## Result

You can compare my old [`init.vim`](https://github.com/khuedoan/dotfiles/blob/76c88283c86e822672f02e9e0e73344a69a91dc1/.config/nvim/init.vim) and the new [`init.lua`](https://github.com/khuedoan/dotfiles/tree/master/.config/nvim/init.lua).
