#!/usr/bin/env -S nvim -l

vim.opt.rtp:append(".")
vim.opt.rtp:append("./nvim-web-devicons")

local list = {
    "Cargo.lock",
    "Cargo.toml",
    "README.md",
    "crates/",
    "crates/file_web_devicon/",
    "crates/file_web_devicon/Cargo.toml",
    "crates/file_web_devicon/src/",
    "crates/file_web_devicon/src/main.rs",
    "crates/file_web_devicon_lib/",
    "crates/file_web_devicon_lib/Cargo.toml",
    "crates/file_web_devicon_lib/benches/",
    "crates/file_web_devicon_lib/benches/fd.txt",
    "crates/file_web_devicon_lib/benches/find.txt",
    "crates/file_web_devicon_lib/benches/rg.txt",
    "crates/file_web_devicon_lib/benches/stress_test.rs",
    "crates/file_web_devicon_lib/src/",
    "crates/file_web_devicon_lib/src/filepaths.rs",
    "crates/file_web_devicon_lib/src/icons.rs",
    "crates/file_web_devicon_lib/src/lib.rs",
    "crates/file_web_devicon_lib/src/tests.rs",
    "lua/",
    "lua/file-web-devicons.lua",
    "nvim-web-devicons/",
    "nvim-web-devicons/CONTRIBUTING.md",
    "nvim-web-devicons/LICENSE",
    "nvim-web-devicons/Makefile",
    "nvim-web-devicons/README.md",
    "nvim-web-devicons/lua/",
    "nvim-web-devicons/lua/nvim-web-devicons/",
    "nvim-web-devicons/lua/nvim-web-devicons/hi-test.lua",
    "nvim-web-devicons/lua/nvim-web-devicons/icons-default.lua",
    "nvim-web-devicons/lua/nvim-web-devicons/icons-light.lua",
    "nvim-web-devicons/lua/nvim-web-devicons.lua",
    "nvim-web-devicons/plugin/",
    "nvim-web-devicons/plugin/nvim-web-devicons.vim",
    "nvim-web-devicons/scripts/",
    "nvim-web-devicons/scripts/filetype-generator.sh",
    "nvim-web-devicons/scripts/generate_colors.lua",
    "scripts/",
    "scripts/bench_ffi.lua",
    "scripts/extract-icons.lua",
    "scripts/update-icons.sh",
}

vim.print(require("nvim-web-devicons"))
local main = function()
    local get
    if arg[1] == "nvim-web-devicons" then
        get = require(arg[1]).get_icons
    elseif arg[1] == "file-web-devicons" then
        get = require(arg[1]).get_icons
    end

    for _, item in ipairs(list) do
        get(item)
        print(item)
    end

    for item in
        vim.gsplit(vim.system({ "fd", ".", vim.env.HOME .. "/b/linux" }):wait().stdout, "\n")
    do
        get(item)
        print(item)
    end
end

local argv = vim.fn.argv()
local ok, msg = pcall(main, argv)
if not ok then
    print(msg)
end

vim.cmd.quit()
