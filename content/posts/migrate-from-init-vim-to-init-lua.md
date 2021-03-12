---
title: Migrate from init.vim to init.lua
date: 2021-03-12T11:59:12+07:00
cover:
  image: TODO
tags:
  - vim
draft: true
---

With Neovim 0.5 support for `init.lua`, here's a quick guide to help someone with no Lua like me to save a few hours :moon:

## Config path

`~/.config/nvim/init.vim` -> `~/.config/nvim/init.lua` 

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
```

```lua
-- init.lua
vim.o.mouse = 'a'     -- global option
vim.bo.tabstop = 4    -- buffer option
vim.wo.number = true  -- window option
```

To see which namespace to use, check the help page for that option

`:help mouse`

```help
'mouse'         string  (default "")
                global             <-- use vim.o
```

`:help tabstop`

```help
'tabstop' 'ts'  number  (default 8)
                local to buffer    <-- use vim.bo
```

`:help number`

```help
'number' 'nu'   boolean (default off)
                local to window    <-- use vim.wo
```

## Global variable

Simmilar to the above

```viml
" init.vim
let g:mapleader = ' '
```

```lua
-- init.lua
vim.g.mapleader = ' '
```

The one with namespace, Lua doesn't support `#` so you have to put them in quote and brackets

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

You can keep using vim-plug, but if you want Lua, paq-nvim feels right at home.

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

```viml
" init.vim
Plug 'joshdick/onedark.vim'
```

```lua
-- init.lua
plug 'joshdick/onedark.vim'
```

The commands is very similar too:

- `:PlugInstall` -> `:PaqInstall`
- `:PlugClean` -> `:PaqClean`

## Result

You can compare my old [`init.vim`](https://github.com/khuedoan/dotfiles/blob/76c88283c86e822672f02e9e0e73344a69a91dc1/.config/nvim/init.vim) and the new [`init.lua`](https://github.com/khuedoan/dotfiles/tree/master/.config/nvim/init.lua).
