I am writing this tutorial for some of my friends who are learning Neovim.
Most beginners need a usable configuration to be effective at their job, but it also should not be too complex so they can understand and extend it.
It also serves as an exercise for me to filter out what is truly important in my setup.

For reference, a skeleton for this blog post is available [here](https://github.com/khuedoan/nvim-minimal),
and you can also view my full config [here](https://github.com/khuedoan/dotfiles/tree/master/.config/nvim).

## Understand the directory structure

By default, Neovim will look for a configuration file at `~/.config/nvim/init.lua`, you can create it with:

```sh
mkdir -p ~/.config/nvim
touch ~/.config/nvim/init.lua
```

In this tutorial, we will only use a single `init.lua` file.
However, you should know that you can split your config into multiple modules later by creating a `lua/` subdirectory, for example:

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

Again, you only need a single `init.lua` file when you first start, so keep it simple and don't split the config file until you need to.

## Sensible options that (almost) everyone can agree on

Show line numbers and enable relative numbers so you can jump to another line on the screen using relative counts like `5j` or `5k`:

```lua
vim.opt.number = true
vim.opt.relativenumber = true
```

We also want the ability to undo after we exit and reopen the file:

```lua
vim.opt.undofile = true
```

Better window splitting behaviour (the default is fine, but most people will find this more natural since it mimics how they read a normal document - top to bottom, left to right):

```lua
vim.opt.splitbelow = true
vim.opt.splitright = true
```

The default leader key is `\`, but using the space bar is way more convenient:

```lua
vim.g.mapleader = " "
```

This one is a bit controversial, but 4 spaces is the sweet spot for tab size, and we expand the tab key to insert spaces instead:

```lua
vim.opt.expandtab = true
vim.opt.tabstop = 4
vim.opt.shiftwidth = 0 -- set to 0 to default to tabstop value
```

Additionally or alternatively, you can use the [vim-sleuth](https://github.com/tpope/vim-sleuth) plugin below to heuristically determine those values based on your project.

## A good plugin manager

[lazy.nvim](https://github.com/folke/lazy.nvim) is by far the best plugin manager for Neovim.
It's fast, easy to use, feature-rich, and unmatched in its ability to lazy-load plugins for much better performance (hence the name).

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

This is the most important one. If you can only install one plugin, choose this.

There are two great fuzzy finders for Neovim at the moment: [fzf.vim](https://github.com/junegunn/fzf.vim) and [telescope.nvim](https://github.com/nvim-telescope/telescope.nvim).
There are trade-offs between the two. Telescope integrates better with Neovim and other plugins, but fzf is way faster.
Performance is more important to me, so I choose fzf for this purpose:

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

You can see the power of `lazy.nvim` in this very first example.
The `fzf.vim` plugin only loads if you press the shortcut, so your Neovim startup will always be fast, and your plugin is only loaded when you need it.

You can verify this by typing `:Files` when you first start Neovim, then press Space twice to open the file finder, close it with `q`, then type `:Files` again.
The first time it will error because the plugin is not loaded yet, but the second time it will work.
There are more lazy loading optimizations in the next sections.

### File manager

A fuzzy finder is good for opening or searching for existing content, but if you want to create, update, or delete files, a file manager is a better choice.

There are multiple file managers for Neovim, but the ones that are the most natural to use are the ones that allow you to manipulate the file system as if it's a Vim buffer, so you don't have to remember additional commands when you update the file system.

There are two contenders for this: [oil.nvim](https://github.com/stevearc/oil.nvim) and [mini.files](https://github.com/echasnovski/mini.files).
Both are viable, but I find the column view of `mini.files` a bit distracting, so I use `oil.nvim`:

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

Now when you press the `-` key, it will show you the current directory.
Keep pressing `-` to go up, or Enter to go down the directory tree.

You can use the normal Vim editing commands to update the file structure (e.g., `ciw` to change the word, `dd` to delete a file, or `p` to paste that file to another location), then save with `:w` to confirm.

### Common editing features

- Automatically insert closing bracket using [nvim-autopairs](https://github.com/windwp/nvim-autopairs) (optional, some people prefer closing them manually):

```lua
    {
        "https://github.com/windwp/nvim-autopairs",
        event = "InsertEnter", -- Only load when you enter Insert mode
        config = function()
            require("nvim-autopairs").setup()
        end,
    },
```

- Toggle comments with `gc` using [Comment.nvim](https://github.com/numToStr/Comment.nvim) (you can use visual block `<C-v>` to comment multiple lines, but this is more convenient):

```lua
    {
        "https://github.com/numToStr/Comment.nvim",
        event = "VeryLazy", -- Special lazy.nvim event for things that can load later and are not important for the initial UI
        config = function()
            require("Comment").setup()
        end,
    },
```

- Automatically determine indent settings using [vim-sleuth](https://github.com/tpope/vim-sleuth) (useful if you work with multiple languages that have different indent styles, such as Python and Go):

```lua
    {
        "https://github.com/tpope/vim-sleuth",
        event = { "BufReadPost", "BufNewFile" }, -- Load after your file content
    },
```

## "IntelliSense"

If you want modern IDE capabilities (autocomplete, go to definition, rename variable, code snippets, etc.), you'll need to set up Language Server Protocol (LSP) integration.

By far this is the most complex thing to set up, so please bear with me.
I recommend [lsp-zero.nvim](https://github.com/VonHeikemen/lsp-zero.nvim) for beginners to avoid a bunch of boilerplate:

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

There's a lot going on in the above config, so I'll explain it a bit.
Several plugins work together to handle the installation, configuration, and integration of language servers with Neovim:

- `mason`: simplifies the management and installation of LSP servers (you can easily add more servers to the `ensure_installed` list)
- `lspconfig`: the official Neovim plugin for configuring LSP servers
- `cmp`: a completion plugin that integrates with LSP servers to provide intelligent code suggestions and autocompletions

Some basic keybindings:

- `<C-n>`: next item in the completion menu
- `<C-p>`: previous item in the completion menu
- `<C-y>`: confirms selection
- `<C-e>`: cancel the completion
- `gd`: go to definition
- `gi`: go to implementation
- See [here](https://github.com/VonHeikemen/lsp-zero.nvim/blob/v3.x/doc/md/lsp.md) for the full list and how to customize your keymaps

This should be sufficient to get you started. After you've mastered the basics, consider the following suggestions:

- [Add more snippets](https://github.com/VonHeikemen/lsp-zero.nvim/blob/v3.x/doc/md/autocomplete.md#add-an-external-collection-of-snippets)
- Consider [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) for better syntax highlighting and additional text objects
- When you require more customization, you can remove `lsp-zero.nvim` and build a custom LSP setup

At this stage, your Neovim configuration should be sufficient for daily use.
Keep reading if you want more!

## Quality of life improvements

While you don't need these plugins to edit effectively, they can certainly make your life easier:

- Reopens files at your last edit position using [vim-lastplace](https://github.com/farmergreg/vim-lastplace):

```lua
    {
        "https://github.com/farmergreg/vim-lastplace",
        event = "BufReadPost",
    },
```

- "Sneak" to any position on the screen with `s`/`S` to move much faster using [leap.nvim](https://github.com/ggandor/leap.nvim):

```lua
    {
        "https://github.com/ggandor/leap.nvim",
        event = "VeryLazy",
        config = function()
            require("leap").set_default_keymaps()
        end,
    },
```

- Show indent level using [indent-blankline.nvim](https://github.com/lukas-reineke/indent-blankline.nvim):

```lua
    {
        "https://github.com/lukas-reineke/indent-blankline.nvim",
        event = { "BufReadPost", "BufNewFile" },
        config = function()
            require("ibl").setup()
        end,
    },
```

- Show changed lines in git and current line blame using [gitsigns.nvim](https://github.com/lewis6991/gitsigns.nvim):

```lua
    {
        "https://github.com/lewis6991/gitsigns.nvim",
        event = "VeryLazy",
        config = function()
            require("gitsigns").setup({
                current_line_blame = true,
            })
        end,
    },
```

- Common git operations using [neogit](https://github.com/NeogitOrg/neogit) (very handy for staging partial changes):

```lua
    {
        "https://github.com/NeogitOrg/neogit",
        cmd = "Neogit", -- Only load when you run the Neogit command
        config = function()
            require("neogit").setup()
        end,
    },
```

## Eye candy

Last but not least, it's pretty hard to resist a nice theme!

Personally, I use [onedark.nvim](https://github.com/navarasu/onedark.nvim) as my daily driver.
But color is very subjective, so you can choose whatever you want.

If you don't want to install any additional plugins, this built-in colorscheme is pretty good:

```lua
-- There's no Lua API to select a colorscheme yet, so we'll call a Vim command
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

After building the essential pieces of your config, just keep using Neovim and get better.
The most important part is to try to note the inefficiencies in your day-to-day workflow and find a way to improve it, either with built-in features or additional plugins.
There are some extremely valuable videos on that subject:

- [7 Habits For Effective Text Editing](https://www.youtube.com/watch?v=eX9m3g5J-XA) by Bram Moolenaar - the creator of Vim (great respect, RIP)
- [How to Do 90% of What Plugins Do](https://www.youtube.com/watch?v=XA2WjJbmmoM) by Max Cantor

Trust the process, keep improving, and I promise you will be faster in Neovim than any other IDE could ever dream of.
