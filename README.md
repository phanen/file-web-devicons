# file-web-devicons

Simple Rust binary that reads lines from `stdin`, and prepends icons as defined
in the excellent
[nvim-web-devicons](https://github.com/nvim-tree/nvim-web-devicons).

Handles color codes as well, using the icon colors from the aforementioned
library. Intended to take in output from [fd](https://github.com/sharkdp/fd) or
[rg](https://github.com/BurntSushi/ripgrep), to
then be used as input for [fzf-lua](https://github.com/ibhagwan/fzf-lua) in
Neovim.

I found that, in large Git projects, `fzf-lua`'s default `files` provider would
have a delay of several seconds before files would show up, however running `fd`
or `fzf` directly would feel much snappier. I concluded (correctly) that this is
because of the extra processing `fzf-lua` is doing to, amongst other things,
prepend the file icon. By using a custom `fzf-lua` action that invokes `fd` and
pipes the results through this binary, you still get the file icons, but with
the snapiness of running `fd`/`fzf` directly. Same benefits for `rg` as well.

You could use this project in Neovim using `fzf-lua` as follows:

```lua
local fzf = require'fzf-lua'
local function fzf_files()
  fzf.fzf_exec('fd --type f --strip-cwd-prefix | /path/to/file-web-devicon', {
    actions = fzf.defaults.actions.files,
    fzf_opts = { ['--nth'] = 2, ['--delimiter'] = fzf.utils.nbsp },
    previewer = 'builtin',
  })
end

vim.keymap.set('n', '<C-p>', fzf_files)

local function fzf_live_grep()
  fzf.fzf_live(
    'rg --column --line-number --no-heading --color=always --smart-case -- <query> | /path/to/file-web-devicon', {
    actions = fzf.defaults.actions.files,
    prompt = 'Rg> ',
    fzf_opts = { ['--nth'] = 2, ['--delimiter'] = fzf.utils.nbsp },
    previewer = 'builtin',
  })
end

vim.keymap.set('n', '<C-s>', fzf_live_grep)
```

_Note_: This project has been tested only on MacOS. I don't know whether it will
work on Windows, Linux, or e.g. with non-UTF8 path inputs.

## perfomance
```
$ hyperfine -w 5 'fd . ~/b/linux | file_web_devicon'
Benchmark 1: fd . ~/b/linux | file_web_devicon
  Time (mean ± σ):      77.2 ms ±   2.9 ms    [User: 255.6 ms, System: 74.5 ms]
  Range (min … max):    71.0 ms …  83.1 ms    37 runs

$ hyperfine -w 5 'scripts/headless_fd.sh -c ~/b/linux'
Benchmark 1: scripts/headless_fd.sh -c ~/b/linux
  Time (mean ± σ):     214.8 ms ±   3.7 ms    [User: 464.1 ms, System: 75.5 ms]
  Range (min … max):   210.5 ms … 222.2 ms    13 runs
```
