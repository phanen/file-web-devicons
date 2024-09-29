#!/usr/bin/env nvim --headless -l

local web_devicons = require("nvim-web-devicons")

local default_icon = web_devicons.get_default_icon()
io.write(
    ('pub(crate) static DEFAULT_ICON: Lazy<Icon> = Lazy::new(|| Icon::new("%s", 0x%s));\n'):format(
        default_icon.icon,
        default_icon.color:upper():sub(2)
    )
)

local maps = { "get_icons_by_filename", "get_icons_by_extension" }

for _, map_name in ipairs(maps) do
    local icons = web_devicons[map_name]()
    assert(icons ~= nil, "icons not found for " .. map_name)
    local upper = map_name:gsub("^get_", ""):upper()

    io.write("\n")
    io.write(("pub(crate) static %s: Lazy<HashMap<&str, Icon>> = Lazy::new(|| {\n"):format(upper))
    local lines = {}
    for k, v in pairs(icons) do
        lines[#lines + 1] = ([[    m.insert("%s", Icon::new("%s", 0x%s));]]):format(
            k,
            v.icon,
            v.color:upper():sub(2)
        )
    end
    table.sort(lines)

    io.write("    let mut m = HashMap::with_capacity(" .. #lines .. ");\n")

    io.write(table.concat(lines, "\n"))
    io.write("\n    m\n")
    io.write("});\n")
end

-- vim: ts=4 et :
