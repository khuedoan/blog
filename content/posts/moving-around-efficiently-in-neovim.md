Today I have some quick tips to help you move around faster in
Neovim. If you're already familiar with the basics of NeoVim and want to
improve your speed, this article is for you. Let's dive right in!

## Horizontal motions

Let's start with the easiest part. You can simply use the standard keys,
nothing special:

- <kbd>f</kbd> / <kbd>t</kbd> to **find** a character or jump **till** a
character, then use <kbd>;</kbd> to repeat if needed
- <kbd>b</kbd> / <kbd>B</kbd> to move to the **begining** of a word or WORD
  (don't worry if you're unsure about the difference between a word and a WORD,
  we'll cover that shortly)
- <kbd>e</kbd> / <kbd>E</kbd> to move to the **end** of a word or WORD
- <kbd>w</kbd> / <kbd>W</kbd> to move to the next word or WORD

Now, what exactly is the difference between a word and a WORD?

A word consists of a sequence of non-blank characters, separated by whitespace.
For example, in the previous sentence, `non` and `blank` are words, while
`non-blank` is a WORD. This feature often goes unnoticed, and many users,
including myself, discovered it years after using Vim.

## Vertical motions

Turn on `number` and `relativenumber`, then you can move up and down with
<kbd>5</kbd> <kbd>j</kbd> and <kbd>5</kbd> <kbd>k</kbd> (replace <kbd>5</kbd>
with the relative line number you want to jump to).

Practicing typing numbers without looking can make your vertical navigation
more efficient.

## Diagonal motions

In vanilla NeoVim, you can use <kbd>/</kbd> to search or combine horizontal and
vertical motions as mentioned earlier, but it can be even faster!

Install [leap.nvim](https://github.com/ggandor/leap.nvim), with this plugin you
can use <kbd>s</kbd> / <kbd>S</kbd> to move to any arbitrary location on the
screen with only a few keystrokes. In most cases, you'll only need 4
keystrokes, and 6 is the worst-case scenario. Alternatively,
[flash.nvim](https://github.com/folke/flash.nvim): is another great option.

## File navigation

The fastest way to open a file in your project, assuming you know its general
location, is to use a fuzzy finder. You can use
[fzf.vim](https://github.com/junegunn/fzf.vim) or
[telescope.nvim](https://github.com/nvim-telescope/telescope.nvim), these fuzzy
finders also allow you to search for any text in your project.

In my config, I just need to hit <kbd>Space</kbd> <kbd>Space</kbd> and search
for any file I want. The best part is that I don't even need to type the path
correctly!

## File exploration and manipulation

A fuzzy finder is not really useful if you just want to explore the project, or
perform actions like renaming, copying, or moving files. For that, I use a
plugin called [oil.nvim](https://github.com/stevearc/oil.nvim). It allows me to
edit the filesystem just like a normal buffer.

To navigate the filesystem, I use the <kbd>-</kbd> key to show the parent
directory of the current file. From there, I can press <kbd>-</kbd> again to go
up or press <kbd>Enter</kbd> to open a directory or file. I can also rename
files, move them around, and then save the changes with <kbd>:</kbd>
<kbd>w</kbd> just like a normal buffer.

That's it! I hope you enjoyed and learned something new today, happy coding!
