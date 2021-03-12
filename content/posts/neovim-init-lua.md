---
title: From init.vim to init.lua
date: 2021-03-12T11:59:12+07:00
cover:
  image: TODO
tags:
  - vim
draft: true
---

Quick guide to migrate your NeoVim config to Lua.

You can checkout my [old `init.vim`](https://github.com/khuedoan/dotfiles/tree/master/.config/nvim.old/init.vim) and the [new `init.lua`](https://github.com/khuedoan/dotfiles/tree/master/.config/nvim/init.lua)

## Config path

`~/.config/nvim/init.vim` -> `~/.config/nvim/init.lua` 

## Comment

`init.vim`

```viml
" This is a comment
```

`init.lua`

```lua
-- This is a comment
```

## Options

Options now include namespace

Old `init.vim`

```viml
set mouse=a
set tabstop=4
set number
```

New `init.lua`

```lua
vim.o.mouse = 'a'
vim.bo.tabstop = 4
vim.wo.number = true
```

To see which namespace to use, check the help page for that option

`:help mouse`

```txt
'mouse'         string  (default "")
                global  <-- This is the namespace
```

## Key mappings

```viml
nnoremap <C-s> :write<CR>
```

```lua
--                       <mode>  <keys>    <actions>       <options>
vim.api.nvim_set_keymap( 'n',   '<C-s>', ':write<CR>', {noremap = true})
```

## Plugin manager

Use paq-nvim instead of vim-plug

```lua
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

Then install your plugins

```lua
plug {'joshdick/onedark.vim'}
```

## Set variable

```viml
let g:fzf_buffers_jump = 1
```

```lua
vim.g.fzf_buffers_jump = 1
```

The one with namespace, Lua doesn't support `#` so you have to put them in quote and brackets

```viml
let g:sneak#label = 1
```

```lua
vim.g['sneak#label'] = 1
```
