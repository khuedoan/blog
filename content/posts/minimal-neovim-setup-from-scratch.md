Some of my friends who want to start using Neovim asked me for pointers to get
started, so I decided to write a blog post for future reference. Most beginners
need a usable configuration to be effective at their job, but it should also be
simple enough to understand and extend. This is also a great opportunity for me
to filter out what’s truly important in my setup (spoiler: I cut out one-third
of my plugins after writing this post).

For reference, a skeleton for this blog post is available
[here](https://github.com/khuedoan/nvim-minimal). Since this post may become
outdated, also check out my full configuration
[here](https://github.com/khuedoan/dotfiles/tree/master/.config/nvim) for the
latest updates.

## Understand the directory structure

By default, Neovim will look for a configuration file at
`~/.config/nvim/init.lua`, create it with:

```sh
mkdir -p ~/.config/nvim
touch ~/.config/nvim/init.lua
```

In this tutorial, we will use a single `init.lua` file. However, you can split
your configuration into multiple modules later by creating a `lua/`
subdirectory, for example:

```
~/.config/nvim
├── init.lua
└── lua
    └── mymodule.lua
```

Then you can include them in `init.lua`:

```lua
require("mymodule")
```

Though you only need a single `init.lua` file when starting out, so keep it
simple and avoid splitting the config file until necessary.

## Sensible options that (almost) everyone can agree on

Enable line numbers and relative numbers to jump to other lines on the screen using relative counts like <kbd>5</kbd> <kbd>j</kbd> or <kbd>5</kbd> <kbd>k</kbd>:

```lua
vim.opt.number = true
vim.opt.relativenumber = true
```

We also want the ability to undo changes after exiting and reopening the file:

```lua
vim.opt.undofile = true
```

Improved window-splitting behavior - while the default is acceptable, most
people find this more natural as it mimics reading a document from top to
bottom and left to right:

```lua
vim.opt.splitbelow = true
vim.opt.splitright = true
```

The default leader key is <kbd>\\</kbd>, but using the <kbd>Space</kbd> bar is way more convenient:

```lua
vim.g.mapleader = " "
```

This is a bit controversial, but 4 spaces is the sweet spot for tab size, and we configure the tab key to insert spaces instead:

```lua
vim.opt.expandtab = true
vim.opt.tabstop = 4
vim.opt.shiftwidth = 0 -- set to 0 to default to tabstop value
```

Additionally, you can use the [vim-sleuth](https://github.com/tpope/vim-sleuth)
plugin below to heuristically determine those values based on your project.

## A good plugin manager

[lazy.nvim](https://github.com/folke/lazy.nvim) is by far the best plugin
manager for Neovim. It's fast, easy to use, feature-rich, and unmatched in its
ability to lazy-load plugins for significantly better performance (hence the
name).

Add the following code block to load `lazy.nvim` (and install it if it does not exist):

```lua
-- Visit the project page for the latest installation instructions
-- https://github.com/folke/lazy.nvim
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
    vim.fn.system({
        "git",
        "clone",
        "--filter=blob:none",
        "https://github.com/folke/lazy.nvim.git",
        "--branch=stable",
        lazypath,
    })
end
vim.opt.rtp:prepend(lazypath)

require("lazy").setup({
    -- Boilerplate for next steps.
    -- From now on, all code examples will go to this section.
    -- {
    --     "https://gitprovider.com/exampleuser/myplugin",
    -- },
})
```

Quit and reopen Neovim to load `lazy.nvim`.
Do the same after adding new plugins to the config file.

## Essential plugins

Now that we have our plugin manager set up, let's install a few essential plugins.

### Fuzzy finder

This is arguably the most important one. If you can only install one plugin,
choose this.

There are two great fuzzy finders for Neovim at the moment:
[fzf.vim](https://github.com/junegunn/fzf.vim) and
[telescope.nvim](https://github.com/nvim-telescope/telescope.nvim). There are
trade-offs between the two. Telescope integrates better with Neovim and other
plugins, but fzf is way faster. Performance is more important to me, so I
choose `fzf` for this purpose:

```lua
    {
        "https://github.com/junegunn/fzf.vim",
        dependencies = {
            "https://github.com/junegunn/fzf",
        },
        keys = {
            { "<Leader><Leader>", "<Cmd>Files<CR>", desc = "Find files" },
            { "<Leader>,", "<Cmd>Buffers<CR>", desc = "Find buffers" },
            { "<Leader>/", "<Cmd>Rg<CR>", desc = "Search project" },
        },
    },
```

You can see the power of `lazy.nvim` in this very first example. The `fzf.vim`
plugin only loads if you press the shortcut, so your Neovim startup will always
be fast, and your plugin is only loaded when you need it.

You can verify this by typing `:Files` when you first start Neovim, then press
Space twice to open the file finder, close it with <kbd>q</kbd>, then type
`:Files` again. The first time it will error because the plugin is not loaded
yet, but the second time it will work. There are more lazy loading
optimizations in the next sections.

### File manager

A fuzzy finder is good for opening or searching for existing content, but if
you want to create, update, or delete files, a file manager is a better choice.

There are multiple file managers for Neovim, but the ones that are the most
natural to use are the ones that allow you to manipulate the file system as if
it's a Vim buffer, so you don't have to remember additional commands when you
update the file system.

There are two contenders for this:
[oil.nvim](https://github.com/stevearc/oil.nvim) and
[mini.files](https://github.com/echasnovski/mini.files). Both are viable, but I
find the column view of `mini.files` a tiny bit distracting, so I use
`oil.nvim`:

```lua
    {
        "https://github.com/stevearc/oil.nvim",
        config = function()
            require("oil").setup()
        end,
        keys = {
            { "-", "<Cmd>Oil<CR>", desc = "Browse files from here" },
        },
    },
```

Now when you press the <kbd>-</kbd> key, it will show you the current
directory. Keep pressing <kbd>-</kbd> to go up, or <kbd>Enter</kbd> to go down
the directory tree.

You can use the normal Vim editing commands to update the file structure (e.g.,
<kbd>c</kbd> <kbd>i</kbd> <kbd>w</kbd> to change in word, <kbd>d</kbd>
<kbd>d</kbd> to delete a file, or <kbd>p</kbd> to paste that file to another
location), then save with <kbd>:</kbd> <kbd>w</kbd> to confirm.

### Common editing features

Automatically insert closing bracket using
[nvim-autopairs](https://github.com/windwp/nvim-autopairs) (optional, some
people prefer closing them manually):

```lua
    {
        "https://github.com/windwp/nvim-autopairs",
        event = "InsertEnter", -- Only load when you enter Insert mode
        config = function()
            require("nvim-autopairs").setup()
        end,
    },
```

Toggle comments with <kbd>g</kbd> <kbd>c</kbd> using
[Comment.nvim](https://github.com/numToStr/Comment.nvim) (you can use visual
block <kbd>Ctrl + v</kbd> to comment multiple lines, but this is more
convenient):

```lua
    {
        "https://github.com/numToStr/Comment.nvim",
        event = "VeryLazy", -- Special lazy.nvim event for things that can load later and are not important for the initial UI
        config = function()
            require("Comment").setup()
        end,
    },
```

Automatically determine indent settings using
[vim-sleuth](https://github.com/tpope/vim-sleuth) (useful if you work with
multiple languages that have different indent styles, such as Python and Go):

```lua
    {
        "https://github.com/tpope/vim-sleuth",
        event = { "BufReadPost", "BufNewFile" }, -- Load after your file content
    },
```

## "IntelliSense"

If you want modern IDE capabilities (autocomplete, go to definition, rename
variable, code snippets, etc.), you'll need to set up Language Server Protocol
(LSP) integration.

By far this is the most complex thing to set up, so please bear with me. I
recommend [lsp-zero.nvim](https://github.com/VonHeikemen/lsp-zero.nvim) for
beginners to avoid a bunch of boilerplate:

```lua
    {
        "https://github.com/VonHeikemen/lsp-zero.nvim",
        dependencies = {
            "https://github.com/williamboman/mason.nvim",
            "https://github.com/williamboman/mason-lspconfig.nvim",
            "https://github.com/neovim/nvim-lspconfig",
            "https://github.com/hrsh7th/cmp-nvim-lsp",
            "https://github.com/hrsh7th/nvim-cmp",
            "https://github.com/L3MON4D3/LuaSnip",
        },
        config = function()
            local lsp_zero = require('lsp-zero')

            lsp_zero.on_attach(function(client, bufnr)
                lsp_zero.default_keymaps({buffer = bufnr})
            end)

            require("mason").setup()
            require("mason-lspconfig").setup({
                ensure_installed = {
                    -- See https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md
                    "gopls", -- Go
                    "pyright", -- Python
                    "rust_analyzer", -- Rust
                },
                handlers = {
                    lsp_zero.default_setup,
                },
            })
        end,
    },
```

There's a lot going on in the above config, so I'll explain it a bit. Several
plugins work together to handle the installation, configuration, and
integration of language servers with Neovim:

- `mason`: simplifies the management and installation of LSP servers (you can
  easily add more servers to the `ensure_installed` list)
- `lspconfig`: the official Neovim plugin for configuring LSP servers
- `cmp`: a completion plugin that integrates with LSP servers to provide
  intelligent code suggestions and autocompletions

Some basic keybindings:

- <kbd>Ctrl + n</kbd>: next item in the completion menu
- <kbd>Ctrl + p</kbd>: previous item in the completion menu
- <kbd>Ctrl + y</kbd>: confirms selection
- <kbd>Ctrl + e</kbd>: cancel the completion
- <kbd>g</kbd> <kbd>d</kbd>: go to definition
- <kbd>g</kbd> <kbd>i</kbd>: go to implementation
- See [here](https://github.com/VonHeikemen/lsp-zero.nvim/blob/v3.x/doc/md/lsp.md) for the full list and how to customize your keymaps

This should be sufficient to get you started. After you've mastered the basics,
consider the following suggestions:

- [Add more
  snippets](https://github.com/VonHeikemen/lsp-zero.nvim/blob/v3.x/doc/md/autocomplete.md#add-an-external-collection-of-snippets)
- Consider
  [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) for
  better syntax highlighting and additional text objects
- When you require more customization, you can remove `lsp-zero.nvim` and build
  a custom LSP setup

At this stage, your Neovim configuration should be sufficient for daily use.
Keep reading if you want more!

## Quality of life improvements

Reopens files at your last edit position using
[vim-lastplace](https://github.com/farmergreg/vim-lastplace):

```lua
    {
        "https://github.com/farmergreg/vim-lastplace",
        event = "BufReadPost",
    },
```

Flash a label to quickly jump to a search result using
[flash.nvim](https://github.com/folke/flash.nvim):

```lua
    {
        "https://github.com/folke/flash.nvim",
        event = "VeryLazy",
        config = function()
            require("flash").setup({
                modes = {
                    search = {
                        enabled = true,
                    },
                    char = {
                        enabled = false,
                    },
                },
            })
        end,
    },
```

Show indent level using
[indent-blankline.nvim](https://github.com/lukas-reineke/indent-blankline.nvim):

```lua
    {
        "https://github.com/lukas-reineke/indent-blankline.nvim",
        event = { "VeryLazy" },
        config = function()
            require("ibl").setup()
        end,
    },
```

Common git operations using [neogit](https://github.com/NeogitOrg/neogit)
(very handy for staging partial changes):

```lua
    {
        "https://github.com/NeogitOrg/neogit",
        cmd = "Neogit", -- Only load when you run the Neogit command
        config = function()
            require("neogit").setup()
        end,
    },
```

## Some eye candies

Last but not least, it's pretty hard to resist a nice theme!

Personally, I use [onedark.nvim](https://github.com/navarasu/onedark.nvim) as
my daily driver. But color is very subjective, so you can choose whatever you
want.

If you don't want to install any additional plugins, some built-in colorschemes
looks good too:

```lua
-- There's no Lua API to select a colorscheme yet, so we'll call a Vim command
-- On Neovim 0.10.0 and above, the default colorscheme is quite usable
vim.cmd("colorscheme default") -- It's already the default, so you don't need to call this
vim.cmd("colorscheme habamax") -- Available in Neovim >= 0.9.0
```

If you're feeling fancy, throw in a nice status line too:

```lua
    {
        "https://github.com/nvim-lualine/lualine.nvim",
        event = "VeryLazy",
        config = function()
            require("lualine").setup()
        end,
    },
```

## What next?

After building the essential parts of your config, keep using Neovim and get
better. The key is to identify inefficiencies in your day-to-day workflow and
find ways to improve them, either with built-in features or additional plugins.
There are some highly valuable videos on this subject:

- [7 Habits For Effective Text
  Editing](https://www.youtube.com/watch?v=eX9m3g5J-XA) by Bram Moolenaar - the
  creator of Vim (great respect, RIP)
- [How to Do 90% of What Plugins
  Do](https://www.youtube.com/watch?v=XA2WjJbmmoM) by Max Cantor

Trust the process, keep improving, and I promise you'll be faster in Neovim
than in any other IDE.
