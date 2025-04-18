use crate::Icon;
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Source for all the icon definitions below:
// https://github.com/nvim-tree/nvim-web-devicons/blob/master/lua/nvim-web-devicons.lua
// Use `scripts/update-icons.sh` to update the icons.

// BEGIN GENERATED CODE
pub(crate) static DEFAULT_ICON: Lazy<Icon> = Lazy::new(|| Icon::new("", 0x6D8086));

pub(crate) static ICONS_BY_FILENAME: Lazy<HashMap<&str, Icon>> = Lazy::new(|| {
    let mut m = HashMap::with_capacity(167);
    m.insert(".SRCINFO", Icon::new("󰣇", 0x0F94D2));
    m.insert(".Xauthority", Icon::new("", 0xE54D18));
    m.insert(".Xresources", Icon::new("", 0xE54D18));
    m.insert(".babelrc", Icon::new("", 0xCBCB41));
    m.insert(".bash_profile", Icon::new("", 0x89E051));
    m.insert(".bashrc", Icon::new("", 0x89E051));
    m.insert(".dockerignore", Icon::new("󰡨", 0x458EE6));
    m.insert(".ds_store", Icon::new("", 0x41535B));
    m.insert(".editorconfig", Icon::new("", 0xFFF2F2));
    m.insert(".env", Icon::new("", 0xFAF743));
    m.insert(".eslintignore", Icon::new("", 0x4B32C3));
    m.insert(".eslintrc", Icon::new("", 0x4B32C3));
    m.insert(".git-blame-ignore-revs", Icon::new("", 0xF54D27));
    m.insert(".gitattributes", Icon::new("", 0xF54D27));
    m.insert(".gitconfig", Icon::new("", 0xF54D27));
    m.insert(".gitignore", Icon::new("", 0xF54D27));
    m.insert(".gitlab-ci.yml", Icon::new("", 0xE24329));
    m.insert(".gitmodules", Icon::new("", 0xF54D27));
    m.insert(".gtkrc-2.0", Icon::new("", 0xFFFFFF));
    m.insert(".gvimrc", Icon::new("", 0x019833));
    m.insert(".justfile", Icon::new("", 0x6D8086));
    m.insert(".luaurc", Icon::new("", 0x00A2FF));
    m.insert(".mailmap", Icon::new("󰊢", 0xF54D27));
    m.insert(".npmignore", Icon::new("", 0xE8274B));
    m.insert(".npmrc", Icon::new("", 0xE8274B));
    m.insert(".nuxtrc", Icon::new("󱄆", 0x00C58E));
    m.insert(".nvmrc", Icon::new("", 0x5FA04E));
    m.insert(".prettierignore", Icon::new("", 0x4285F4));
    m.insert(".prettierrc", Icon::new("", 0x4285F4));
    m.insert(".prettierrc.cjs", Icon::new("", 0x4285F4));
    m.insert(".prettierrc.js", Icon::new("", 0x4285F4));
    m.insert(".prettierrc.json", Icon::new("", 0x4285F4));
    m.insert(".prettierrc.json5", Icon::new("", 0x4285F4));
    m.insert(".prettierrc.mjs", Icon::new("", 0x4285F4));
    m.insert(".prettierrc.toml", Icon::new("", 0x4285F4));
    m.insert(".prettierrc.yaml", Icon::new("", 0x4285F4));
    m.insert(".prettierrc.yml", Icon::new("", 0x4285F4));
    m.insert(".settings.json", Icon::new("", 0x854CC7));
    m.insert(".vimrc", Icon::new("", 0x019833));
    m.insert(".xinitrc", Icon::new("", 0xE54D18));
    m.insert(".xsession", Icon::new("", 0xE54D18));
    m.insert(".zprofile", Icon::new("", 0x89E051));
    m.insert(".zshenv", Icon::new("", 0x89E051));
    m.insert(".zshrc", Icon::new("", 0x89E051));
    m.insert("FreeCAD.conf", Icon::new("", 0xCB333B));
    m.insert("PKGBUILD", Icon::new("", 0x0F94D2));
    m.insert("PrusaSlicer.ini", Icon::new("", 0xEC6B23));
    m.insert("PrusaSlicerGcodeViewer.ini", Icon::new("", 0xEC6B23));
    m.insert("QtProject.conf", Icon::new("", 0x40CD52));
    m.insert("R", Icon::new("󰟔", 0x2266BA));
    m.insert("_gvimrc", Icon::new("", 0x019833));
    m.insert("_vimrc", Icon::new("", 0x019833));
    m.insert("avif", Icon::new("", 0xA074C4));
    m.insert("brewfile", Icon::new("", 0x701516));
    m.insert("bspwmrc", Icon::new("", 0x2F2F2F));
    m.insert("build", Icon::new("", 0x89E051));
    m.insert("build.gradle", Icon::new("", 0x005F87));
    m.insert("build.zig.zon", Icon::new("", 0xF69A1B));
    m.insert("cantorrc", Icon::new("", 0x1C99F3));
    m.insert("checkhealth", Icon::new("󰓙", 0x75B4FB));
    m.insert("cmakelists.txt", Icon::new("", 0x6D8086));
    m.insert("code_of_conduct", Icon::new("", 0xE41662));
    m.insert("code_of_conduct.md", Icon::new("", 0xE41662));
    m.insert("commit_editmsg", Icon::new("", 0xF54D27));
    m.insert("commitlint.config.js", Icon::new("󰜘", 0x2B9689));
    m.insert("commitlint.config.ts", Icon::new("󰜘", 0x2B9689));
    m.insert("compose.yaml", Icon::new("󰡨", 0x458EE6));
    m.insert("compose.yml", Icon::new("󰡨", 0x458EE6));
    m.insert("config", Icon::new("", 0x6D8086));
    m.insert("containerfile", Icon::new("󰡨", 0x458EE6));
    m.insert("copying", Icon::new("", 0xCBCB41));
    m.insert("copying.lesser", Icon::new("", 0xCBCB41));
    m.insert("docker-compose.yaml", Icon::new("󰡨", 0x458EE6));
    m.insert("docker-compose.yml", Icon::new("󰡨", 0x458EE6));
    m.insert("dockerfile", Icon::new("󰡨", 0x458EE6));
    m.insert("eslint.config.cjs", Icon::new("", 0x4B32C3));
    m.insert("eslint.config.js", Icon::new("", 0x4B32C3));
    m.insert("eslint.config.mjs", Icon::new("", 0x4B32C3));
    m.insert("eslint.config.ts", Icon::new("", 0x4B32C3));
    m.insert("ext_typoscript_setup.txt", Icon::new("", 0xFF8700));
    m.insert("favicon.ico", Icon::new("", 0xCBCB41));
    m.insert("fp-info-cache", Icon::new("", 0xFFFFFF));
    m.insert("fp-lib-table", Icon::new("", 0xFFFFFF));
    m.insert("gemfile$", Icon::new("", 0x701516));
    m.insert("gnumakefile", Icon::new("", 0x6D8086));
    m.insert("go.mod", Icon::new("", 0x519ABA));
    m.insert("go.sum", Icon::new("", 0x519ABA));
    m.insert("go.work", Icon::new("", 0x519ABA));
    m.insert("gradle-wrapper.properties", Icon::new("", 0x005F87));
    m.insert("gradle.properties", Icon::new("", 0x005F87));
    m.insert("gradlew", Icon::new("", 0x005F87));
    m.insert("groovy", Icon::new("", 0x4A687C));
    m.insert("gruntfile.babel.js", Icon::new("", 0xE37933));
    m.insert("gruntfile.coffee", Icon::new("", 0xE37933));
    m.insert("gruntfile.js", Icon::new("", 0xE37933));
    m.insert("gruntfile.ts", Icon::new("", 0xE37933));
    m.insert("gtkrc", Icon::new("", 0xFFFFFF));
    m.insert("gulpfile.babel.js", Icon::new("", 0xCC3E44));
    m.insert("gulpfile.coffee", Icon::new("", 0xCC3E44));
    m.insert("gulpfile.js", Icon::new("", 0xCC3E44));
    m.insert("gulpfile.ts", Icon::new("", 0xCC3E44));
    m.insert("hypridle.conf", Icon::new("", 0x00AAAE));
    m.insert("hyprland.conf", Icon::new("", 0x00AAAE));
    m.insert("hyprlock.conf", Icon::new("", 0x00AAAE));
    m.insert("hyprpaper.conf", Icon::new("", 0x00AAAE));
    m.insert("i18n.config.js", Icon::new("󰗊", 0x7986CB));
    m.insert("i18n.config.ts", Icon::new("󰗊", 0x7986CB));
    m.insert("i3blocks.conf", Icon::new("", 0xE8EBEE));
    m.insert("i3status.conf", Icon::new("", 0xE8EBEE));
    m.insert("ionic.config.json", Icon::new("", 0x4F8FF7));
    m.insert("justfile", Icon::new("", 0x6D8086));
    m.insert("kalgebrarc", Icon::new("", 0x1C99F3));
    m.insert("kdeglobals", Icon::new("", 0x1C99F3));
    m.insert("kdenlive-layoutsrc", Icon::new("", 0x83B8F2));
    m.insert("kdenliverc", Icon::new("", 0x83B8F2));
    m.insert("kritadisplayrc", Icon::new("", 0xF245FB));
    m.insert("kritarc", Icon::new("", 0xF245FB));
    m.insert("license", Icon::new("", 0xD0BF41));
    m.insert("license.md", Icon::new("", 0xD0BF41));
    m.insert("lxde-rc.xml", Icon::new("", 0x909090));
    m.insert("lxqt.conf", Icon::new("", 0x0192D3));
    m.insert("makefile", Icon::new("", 0x6D8086));
    m.insert("mix.lock", Icon::new("", 0xA074C4));
    m.insert("mpv.conf", Icon::new("", 0x3B1342));
    m.insert("node_modules", Icon::new("", 0xE8274B));
    m.insert("nuxt.config.cjs", Icon::new("󱄆", 0x00C58E));
    m.insert("nuxt.config.js", Icon::new("󱄆", 0x00C58E));
    m.insert("nuxt.config.mjs", Icon::new("󱄆", 0x00C58E));
    m.insert("nuxt.config.ts", Icon::new("󱄆", 0x00C58E));
    m.insert("package-lock.json", Icon::new("", 0x7A0D21));
    m.insert("package.json", Icon::new("", 0xE8274B));
    m.insert("platformio.ini", Icon::new("", 0xF6822B));
    m.insert("pom.xml", Icon::new("", 0x7A0D21));
    m.insert("prettier.config.cjs", Icon::new("", 0x4285F4));
    m.insert("prettier.config.js", Icon::new("", 0x4285F4));
    m.insert("prettier.config.mjs", Icon::new("", 0x4285F4));
    m.insert("prettier.config.ts", Icon::new("", 0x4285F4));
    m.insert("procfile", Icon::new("", 0xA074C4));
    m.insert("py.typed", Icon::new("", 0xFFBC03));
    m.insert("r", Icon::new("󰟔", 0x2266BA));
    m.insert("rakefile", Icon::new("", 0x701516));
    m.insert("rmd", Icon::new("", 0x519ABA));
    m.insert("robots.txt", Icon::new("󰚩", 0x5D7096));
    m.insert("security", Icon::new("󰒃", 0xBEC4C9));
    m.insert("security.md", Icon::new("󰒃", 0xBEC4C9));
    m.insert("settings.gradle", Icon::new("", 0x005F87));
    m.insert("svelte.config.js", Icon::new("", 0xFF3E00));
    m.insert("sxhkdrc", Icon::new("", 0x2F2F2F));
    m.insert("sym-lib-table", Icon::new("", 0xFFFFFF));
    m.insert("tailwind.config.js", Icon::new("󱏿", 0x20C2E3));
    m.insert("tailwind.config.mjs", Icon::new("󱏿", 0x20C2E3));
    m.insert("tailwind.config.ts", Icon::new("󱏿", 0x20C2E3));
    m.insert("tmux.conf", Icon::new("", 0x14BA19));
    m.insert("tmux.conf.local", Icon::new("", 0x14BA19));
    m.insert("tsconfig.json", Icon::new("", 0x519ABA));
    m.insert("unlicense", Icon::new("", 0xD0BF41));
    m.insert("vagrantfile$", Icon::new("", 0x1563FF));
    m.insert("vercel.json", Icon::new("▲", 0xFFFFFF));
    m.insert("vlcrc", Icon::new("󰕼", 0xEE7A00));
    m.insert("webpack", Icon::new("󰜫", 0x519ABA));
    m.insert("weston.ini", Icon::new("", 0xFFBB01));
    m.insert("workspace", Icon::new("", 0x89E051));
    m.insert("xmobarrc", Icon::new("", 0xFD4D5D));
    m.insert("xmobarrc.hs", Icon::new("", 0xFD4D5D));
    m.insert("xmonad.hs", Icon::new("", 0xFD4D5D));
    m.insert("xorg.conf", Icon::new("", 0xE54D18));
    m.insert("xsettingsd.conf", Icon::new("", 0xE54D18));
    m
});

pub(crate) static ICONS_BY_EXTENSION: Lazy<HashMap<&str, Icon>> = Lazy::new(|| {
    let mut m = HashMap::with_capacity(435);
    m.insert("3gp", Icon::new("", 0xFD971F));
    m.insert("3mf", Icon::new("󰆧", 0x888888));
    m.insert("7z", Icon::new("", 0xECA517));
    m.insert("Dockerfile", Icon::new("󰡨", 0x458EE6));
    m.insert("a", Icon::new("", 0xDCDDD6));
    m.insert("aac", Icon::new("", 0x00AFFF));
    m.insert("ai", Icon::new("", 0xCBCB41));
    m.insert("aif", Icon::new("", 0x00AFFF));
    m.insert("aiff", Icon::new("", 0x00AFFF));
    m.insert("android", Icon::new("", 0x34A853));
    m.insert("ape", Icon::new("", 0x00AFFF));
    m.insert("apk", Icon::new("", 0x34A853));
    m.insert("apl", Icon::new("⍝", 0xFFA500));
    m.insert("app", Icon::new("", 0x9F0500));
    m.insert("applescript", Icon::new("", 0x6D8085));
    m.insert("asc", Icon::new("󰦝", 0x576D7F));
    m.insert("ass", Icon::new("󰨖", 0xFFB713));
    m.insert("astro", Icon::new("", 0xE23F67));
    m.insert("awk", Icon::new("", 0x4D5A5E));
    m.insert("azcli", Icon::new("", 0x0078D4));
    m.insert("bak", Icon::new("󰁯", 0x6D8086));
    m.insert("bash", Icon::new("", 0x89E051));
    m.insert("bat", Icon::new("", 0xC1F12E));
    m.insert("bazel", Icon::new("", 0x89E051));
    m.insert("bib", Icon::new("󱉟", 0xCBCB41));
    m.insert("bicep", Icon::new("", 0x519ABA));
    m.insert("bicepparam", Icon::new("", 0x9F74B3));
    m.insert("bin", Icon::new("", 0x9F0500));
    m.insert("blade.php", Icon::new("", 0xF05340));
    m.insert("blend", Icon::new("󰂫", 0xEA7600));
    m.insert("blp", Icon::new("󰺾", 0x5796E2));
    m.insert("bmp", Icon::new("", 0xA074C4));
    m.insert("bqn", Icon::new("⎉", 0x2B7067));
    m.insert("brep", Icon::new("󰻫", 0x839463));
    m.insert("bz", Icon::new("", 0xECA517));
    m.insert("bz2", Icon::new("", 0xECA517));
    m.insert("bz3", Icon::new("", 0xECA517));
    m.insert("bzl", Icon::new("", 0x89E051));
    m.insert("c", Icon::new("", 0x599EFF));
    m.insert("c++", Icon::new("", 0xF34B7D));
    m.insert("cache", Icon::new("", 0xFFFFFF));
    m.insert("cast", Icon::new("", 0xFD971F));
    m.insert("cbl", Icon::new("⚙", 0x005CA5));
    m.insert("cc", Icon::new("", 0xF34B7D));
    m.insert("ccm", Icon::new("", 0xF34B7D));
    m.insert("cfg", Icon::new("", 0x6D8086));
    m.insert("cjs", Icon::new("", 0xCBCB41));
    m.insert("clj", Icon::new("", 0x8DC149));
    m.insert("cljc", Icon::new("", 0x8DC149));
    m.insert("cljd", Icon::new("", 0x519ABA));
    m.insert("cljs", Icon::new("", 0x519ABA));
    m.insert("cmake", Icon::new("", 0x6D8086));
    m.insert("cob", Icon::new("⚙", 0x005CA5));
    m.insert("cobol", Icon::new("⚙", 0x005CA5));
    m.insert("coffee", Icon::new("", 0xCBCB41));
    m.insert("conf", Icon::new("", 0x6D8086));
    m.insert("config.ru", Icon::new("", 0x701516));
    m.insert("cow", Icon::new("󰆚", 0x965824));
    m.insert("cp", Icon::new("", 0x519ABA));
    m.insert("cpp", Icon::new("", 0x519ABA));
    m.insert("cppm", Icon::new("", 0x519ABA));
    m.insert("cpy", Icon::new("⚙", 0x005CA5));
    m.insert("cr", Icon::new("", 0xC8C8C8));
    m.insert("crdownload", Icon::new("", 0x44CDA8));
    m.insert("cs", Icon::new("󰌛", 0x596706));
    m.insert("csh", Icon::new("", 0x4D5A5E));
    m.insert("cshtml", Icon::new("󱦗", 0x512BD4));
    m.insert("cson", Icon::new("", 0xCBCB41));
    m.insert("csproj", Icon::new("󰪮", 0x512BD4));
    m.insert("css", Icon::new("", 0x42A5F5));
    m.insert("csv", Icon::new("", 0x89E051));
    m.insert("cts", Icon::new("", 0x519ABA));
    m.insert("cu", Icon::new("", 0x89E051));
    m.insert("cue", Icon::new("󰲹", 0xED95AE));
    m.insert("cuh", Icon::new("", 0xA074C4));
    m.insert("cxx", Icon::new("", 0x519ABA));
    m.insert("cxxm", Icon::new("", 0x519ABA));
    m.insert("d", Icon::new("", 0x427819));
    m.insert("d.ts", Icon::new("", 0xD59855));
    m.insert("dart", Icon::new("", 0x03589C));
    m.insert("db", Icon::new("", 0xDAD8D8));
    m.insert("dconf", Icon::new("", 0xFFFFFF));
    m.insert("desktop", Icon::new("", 0x563D7C));
    m.insert("diff", Icon::new("", 0x41535B));
    m.insert("dll", Icon::new("", 0x4D2C0B));
    m.insert("doc", Icon::new("󰈬", 0x185ABD));
    m.insert("docx", Icon::new("󰈬", 0x185ABD));
    m.insert("dot", Icon::new("󱁉", 0x30638E));
    m.insert("download", Icon::new("", 0x44CDA8));
    m.insert("drl", Icon::new("", 0xFFAFAF));
    m.insert("dropbox", Icon::new("", 0x0061FE));
    m.insert("dump", Icon::new("", 0xDAD8D8));
    m.insert("dwg", Icon::new("󰻫", 0x839463));
    m.insert("dxf", Icon::new("󰻫", 0x839463));
    m.insert("ebook", Icon::new("", 0xEAB16D));
    m.insert("edn", Icon::new("", 0x519ABA));
    m.insert("eex", Icon::new("", 0xA074C4));
    m.insert("ejs", Icon::new("", 0xCBCB41));
    m.insert("el", Icon::new("", 0x8172BE));
    m.insert("elc", Icon::new("", 0x8172BE));
    m.insert("elf", Icon::new("", 0x9F0500));
    m.insert("elm", Icon::new("", 0x519ABA));
    m.insert("eln", Icon::new("", 0x8172BE));
    m.insert("env", Icon::new("", 0xFAF743));
    m.insert("eot", Icon::new("", 0xECECEC));
    m.insert("epp", Icon::new("", 0xFFA61A));
    m.insert("epub", Icon::new("", 0xEAB16D));
    m.insert("erb", Icon::new("", 0x701516));
    m.insert("erl", Icon::new("", 0xB83998));
    m.insert("ex", Icon::new("", 0xA074C4));
    m.insert("exe", Icon::new("", 0x9F0500));
    m.insert("exs", Icon::new("", 0xA074C4));
    m.insert("f#", Icon::new("", 0x519ABA));
    m.insert("f3d", Icon::new("󰻫", 0x839463));
    m.insert("f90", Icon::new("󱈚", 0x734F96));
    m.insert("fbx", Icon::new("󰆧", 0x888888));
    m.insert("fcbak", Icon::new("", 0xCB333B));
    m.insert("fcmacro", Icon::new("", 0xCB333B));
    m.insert("fcmat", Icon::new("", 0xCB333B));
    m.insert("fcparam", Icon::new("", 0xCB333B));
    m.insert("fcscript", Icon::new("", 0xCB333B));
    m.insert("fcstd", Icon::new("", 0xCB333B));
    m.insert("fcstd1", Icon::new("", 0xCB333B));
    m.insert("fctb", Icon::new("", 0xCB333B));
    m.insert("fctl", Icon::new("", 0xCB333B));
    m.insert("fdmdownload", Icon::new("", 0x44CDA8));
    m.insert("fish", Icon::new("", 0x4D5A5E));
    m.insert("flac", Icon::new("", 0x0075AA));
    m.insert("flc", Icon::new("", 0xECECEC));
    m.insert("flf", Icon::new("", 0xECECEC));
    m.insert("fnl", Icon::new("", 0xFFF3D7));
    m.insert("fs", Icon::new("", 0x519ABA));
    m.insert("fsi", Icon::new("", 0x519ABA));
    m.insert("fsscript", Icon::new("", 0x519ABA));
    m.insert("fsx", Icon::new("", 0x519ABA));
    m.insert("gcode", Icon::new("󰐫", 0x1471AD));
    m.insert("gd", Icon::new("", 0x6D8086));
    m.insert("gemspec", Icon::new("", 0x701516));
    m.insert("gif", Icon::new("", 0xA074C4));
    m.insert("git", Icon::new("", 0xF14C28));
    m.insert("glb", Icon::new("", 0xFFB13B));
    m.insert("gleam", Icon::new("", 0xFFAFF3));
    m.insert("gnumakefile", Icon::new("", 0x6D8086));
    m.insert("go", Icon::new("", 0x519ABA));
    m.insert("godot", Icon::new("", 0x6D8086));
    m.insert("gql", Icon::new("", 0xE535AB));
    m.insert("gradle", Icon::new("", 0x005F87));
    m.insert("graphql", Icon::new("", 0xE535AB));
    m.insert("gresource", Icon::new("", 0xFFFFFF));
    m.insert("gv", Icon::new("󱁉", 0x30638E));
    m.insert("gz", Icon::new("", 0xECA517));
    m.insert("h", Icon::new("", 0xA074C4));
    m.insert("haml", Icon::new("", 0xEAEAE1));
    m.insert("hbs", Icon::new("", 0xF0772B));
    m.insert("heex", Icon::new("", 0xA074C4));
    m.insert("hex", Icon::new("", 0x2E63FF));
    m.insert("hh", Icon::new("", 0xA074C4));
    m.insert("hpp", Icon::new("", 0xA074C4));
    m.insert("hrl", Icon::new("", 0xB83998));
    m.insert("hs", Icon::new("", 0xA074C4));
    m.insert("htm", Icon::new("", 0xE34C26));
    m.insert("html", Icon::new("", 0xE44D26));
    m.insert("http", Icon::new("", 0x008EC7));
    m.insert("huff", Icon::new("󰡘", 0x4242C7));
    m.insert("hurl", Icon::new("", 0xFF0288));
    m.insert("hx", Icon::new("", 0xEA8220));
    m.insert("hxx", Icon::new("", 0xA074C4));
    m.insert("ical", Icon::new("", 0x2B2E83));
    m.insert("icalendar", Icon::new("", 0x2B2E83));
    m.insert("ico", Icon::new("", 0xCBCB41));
    m.insert("ics", Icon::new("", 0x2B2E83));
    m.insert("ifb", Icon::new("", 0x2B2E83));
    m.insert("ifc", Icon::new("󰻫", 0x839463));
    m.insert("ige", Icon::new("󰻫", 0x839463));
    m.insert("iges", Icon::new("󰻫", 0x839463));
    m.insert("igs", Icon::new("󰻫", 0x839463));
    m.insert("image", Icon::new("", 0xD0BEC8));
    m.insert("img", Icon::new("", 0xD0BEC8));
    m.insert("import", Icon::new("", 0xECECEC));
    m.insert("info", Icon::new("", 0xFFFFCD));
    m.insert("ini", Icon::new("", 0x6D8086));
    m.insert("ino", Icon::new("", 0x56B6C2));
    m.insert("ipynb", Icon::new("", 0x51A0CF));
    m.insert("iso", Icon::new("", 0xD0BEC8));
    m.insert("ixx", Icon::new("", 0x519ABA));
    m.insert("java", Icon::new("", 0xCC3E44));
    m.insert("jl", Icon::new("", 0xA270BA));
    m.insert("jpeg", Icon::new("", 0xA074C4));
    m.insert("jpg", Icon::new("", 0xA074C4));
    m.insert("js", Icon::new("", 0xCBCB41));
    m.insert("json", Icon::new("", 0xCBCB41));
    m.insert("json5", Icon::new("", 0xCBCB41));
    m.insert("jsonc", Icon::new("", 0xCBCB41));
    m.insert("jsx", Icon::new("", 0x20C2E3));
    m.insert("jwmrc", Icon::new("", 0x0078CD));
    m.insert("jxl", Icon::new("", 0xA074C4));
    m.insert("kbx", Icon::new("󰯄", 0x737672));
    m.insert("kdb", Icon::new("", 0x529B34));
    m.insert("kdbx", Icon::new("", 0x529B34));
    m.insert("kdenlive", Icon::new("", 0x83B8F2));
    m.insert("kdenlivetitle", Icon::new("", 0x83B8F2));
    m.insert("kicad_dru", Icon::new("", 0xFFFFFF));
    m.insert("kicad_mod", Icon::new("", 0xFFFFFF));
    m.insert("kicad_pcb", Icon::new("", 0xFFFFFF));
    m.insert("kicad_prl", Icon::new("", 0xFFFFFF));
    m.insert("kicad_pro", Icon::new("", 0xFFFFFF));
    m.insert("kicad_sch", Icon::new("", 0xFFFFFF));
    m.insert("kicad_sym", Icon::new("", 0xFFFFFF));
    m.insert("kicad_wks", Icon::new("", 0xFFFFFF));
    m.insert("ko", Icon::new("", 0xDCDDD6));
    m.insert("kpp", Icon::new("", 0xF245FB));
    m.insert("kra", Icon::new("", 0xF245FB));
    m.insert("krz", Icon::new("", 0xF245FB));
    m.insert("ksh", Icon::new("", 0x4D5A5E));
    m.insert("kt", Icon::new("", 0x7F52FF));
    m.insert("kts", Icon::new("", 0x7F52FF));
    m.insert("lck", Icon::new("", 0xBBBBBB));
    m.insert("leex", Icon::new("", 0xA074C4));
    m.insert("less", Icon::new("", 0x563D7C));
    m.insert("lff", Icon::new("", 0xECECEC));
    m.insert("lhs", Icon::new("", 0xA074C4));
    m.insert("lib", Icon::new("", 0x4D2C0B));
    m.insert("license", Icon::new("", 0xCBCB41));
    m.insert("liquid", Icon::new("", 0x95BF47));
    m.insert("lock", Icon::new("", 0xBBBBBB));
    m.insert("log", Icon::new("󰌱", 0xDDDDDD));
    m.insert("lrc", Icon::new("󰨖", 0xFFB713));
    m.insert("lua", Icon::new("", 0x51A0CF));
    m.insert("luac", Icon::new("", 0x51A0CF));
    m.insert("luau", Icon::new("", 0x00A2FF));
    m.insert("m", Icon::new("", 0x599EFF));
    m.insert("m3u", Icon::new("󰲹", 0xED95AE));
    m.insert("m3u8", Icon::new("󰲹", 0xED95AE));
    m.insert("m4a", Icon::new("", 0x00AFFF));
    m.insert("m4v", Icon::new("", 0xFD971F));
    m.insert("magnet", Icon::new("", 0xA51B16));
    m.insert("makefile", Icon::new("", 0x6D8086));
    m.insert("markdown", Icon::new("", 0xDDDDDD));
    m.insert("material", Icon::new("󰔉", 0xB83998));
    m.insert("md", Icon::new("", 0xDDDDDD));
    m.insert("md5", Icon::new("󰕥", 0x8C86AF));
    m.insert("mdx", Icon::new("", 0x519ABA));
    m.insert("mint", Icon::new("󰌪", 0x87C095));
    m.insert("mjs", Icon::new("", 0xF1E05A));
    m.insert("mk", Icon::new("", 0x6D8086));
    m.insert("mkv", Icon::new("", 0xFD971F));
    m.insert("ml", Icon::new("", 0xE37933));
    m.insert("mli", Icon::new("", 0xE37933));
    m.insert("mm", Icon::new("", 0x519ABA));
    m.insert("mo", Icon::new("∞", 0x9772FB));
    m.insert("mobi", Icon::new("", 0xEAB16D));
    m.insert("mojo", Icon::new("", 0xFF4C1F));
    m.insert("mov", Icon::new("", 0xFD971F));
    m.insert("mp3", Icon::new("", 0x00AFFF));
    m.insert("mp4", Icon::new("", 0xFD971F));
    m.insert("mpp", Icon::new("", 0x519ABA));
    m.insert("msf", Icon::new("", 0x137BE1));
    m.insert("mts", Icon::new("", 0x519ABA));
    m.insert("mustache", Icon::new("", 0xE37933));
    m.insert("nfo", Icon::new("", 0xFFFFCD));
    m.insert("nim", Icon::new("", 0xF3D400));
    m.insert("nix", Icon::new("", 0x7EBAE4));
    m.insert("nswag", Icon::new("", 0x85EA2D));
    m.insert("nu", Icon::new(">", 0x3AA675));
    m.insert("o", Icon::new("", 0x9F0500));
    m.insert("obj", Icon::new("󰆧", 0x888888));
    m.insert("ogg", Icon::new("", 0x0075AA));
    m.insert("opus", Icon::new("", 0x0075AA));
    m.insert("org", Icon::new("", 0x77AA99));
    m.insert("otf", Icon::new("", 0xECECEC));
    m.insert("out", Icon::new("", 0x9F0500));
    m.insert("part", Icon::new("", 0x44CDA8));
    m.insert("patch", Icon::new("", 0x41535B));
    m.insert("pck", Icon::new("", 0x6D8086));
    m.insert("pcm", Icon::new("", 0x0075AA));
    m.insert("pdf", Icon::new("", 0xB30B00));
    m.insert("php", Icon::new("", 0xA074C4));
    m.insert("pl", Icon::new("", 0x519ABA));
    m.insert("pls", Icon::new("󰲹", 0xED95AE));
    m.insert("ply", Icon::new("󰆧", 0x888888));
    m.insert("pm", Icon::new("", 0x519ABA));
    m.insert("png", Icon::new("", 0xA074C4));
    m.insert("po", Icon::new("", 0x2596BE));
    m.insert("pot", Icon::new("", 0x2596BE));
    m.insert("pp", Icon::new("", 0xFFA61A));
    m.insert("ppt", Icon::new("󰈧", 0xCB4A32));
    m.insert("prisma", Icon::new("", 0x5A67D8));
    m.insert("pro", Icon::new("", 0xE4B854));
    m.insert("ps1", Icon::new("󰨊", 0x4273CA));
    m.insert("psb", Icon::new("", 0x519ABA));
    m.insert("psd", Icon::new("", 0x519ABA));
    m.insert("psd1", Icon::new("󰨊", 0x6975C4));
    m.insert("psm1", Icon::new("󰨊", 0x6975C4));
    m.insert("pub", Icon::new("󰷖", 0xE3C58E));
    m.insert("pxd", Icon::new("", 0x5AA7E4));
    m.insert("pxi", Icon::new("", 0x5AA7E4));
    m.insert("py", Icon::new("", 0xFFBC03));
    m.insert("pyc", Icon::new("", 0xFFE291));
    m.insert("pyd", Icon::new("", 0xFFE291));
    m.insert("pyi", Icon::new("", 0xFFBC03));
    m.insert("pyo", Icon::new("", 0xFFE291));
    m.insert("pyw", Icon::new("", 0x5AA7E4));
    m.insert("pyx", Icon::new("", 0x5AA7E4));
    m.insert("qm", Icon::new("", 0x2596BE));
    m.insert("qml", Icon::new("", 0x40CD52));
    m.insert("qrc", Icon::new("", 0x40CD52));
    m.insert("qss", Icon::new("", 0x40CD52));
    m.insert("query", Icon::new("", 0x90A850));
    m.insert("r", Icon::new("󰟔", 0x2266BA));
    m.insert("rake", Icon::new("", 0x701516));
    m.insert("rar", Icon::new("", 0xECA517));
    m.insert("razor", Icon::new("󱦘", 0x512BD4));
    m.insert("rb", Icon::new("", 0x701516));
    m.insert("res", Icon::new("", 0xCC3E44));
    m.insert("resi", Icon::new("", 0xF55385));
    m.insert("rlib", Icon::new("", 0xDEA584));
    m.insert("rmd", Icon::new("", 0x519ABA));
    m.insert("rproj", Icon::new("󰗆", 0x358A5B));
    m.insert("rs", Icon::new("", 0xDEA584));
    m.insert("rss", Icon::new("", 0xFB9D3B));
    m.insert("sass", Icon::new("", 0xF55385));
    m.insert("sbt", Icon::new("", 0xCC3E44));
    m.insert("sc", Icon::new("", 0xCC3E44));
    m.insert("scad", Icon::new("", 0xF9D72C));
    m.insert("scala", Icon::new("", 0xCC3E44));
    m.insert("scm", Icon::new("󰘧", 0xEEEEEE));
    m.insert("scss", Icon::new("", 0xF55385));
    m.insert("sh", Icon::new("", 0x4D5A5E));
    m.insert("sha1", Icon::new("󰕥", 0x8C86AF));
    m.insert("sha224", Icon::new("󰕥", 0x8C86AF));
    m.insert("sha256", Icon::new("󰕥", 0x8C86AF));
    m.insert("sha384", Icon::new("󰕥", 0x8C86AF));
    m.insert("sha512", Icon::new("󰕥", 0x8C86AF));
    m.insert("sig", Icon::new("λ", 0xE37933));
    m.insert("signature", Icon::new("λ", 0xE37933));
    m.insert("skp", Icon::new("󰻫", 0x839463));
    m.insert("sldasm", Icon::new("󰻫", 0x839463));
    m.insert("sldprt", Icon::new("󰻫", 0x839463));
    m.insert("slim", Icon::new("", 0xE34C26));
    m.insert("sln", Icon::new("", 0x854CC7));
    m.insert("slvs", Icon::new("󰻫", 0x839463));
    m.insert("sml", Icon::new("λ", 0xE37933));
    m.insert("so", Icon::new("", 0xDCDDD6));
    m.insert("sol", Icon::new("", 0x519ABA));
    m.insert("spec.js", Icon::new("", 0xCBCB41));
    m.insert("spec.jsx", Icon::new("", 0x20C2E3));
    m.insert("spec.ts", Icon::new("", 0x519ABA));
    m.insert("spec.tsx", Icon::new("", 0x1354BF));
    m.insert("sql", Icon::new("", 0xDAD8D8));
    m.insert("sqlite", Icon::new("", 0xDAD8D8));
    m.insert("sqlite3", Icon::new("", 0xDAD8D8));
    m.insert("srt", Icon::new("󰨖", 0xFFB713));
    m.insert("ssa", Icon::new("󰨖", 0xFFB713));
    m.insert("ste", Icon::new("󰻫", 0x839463));
    m.insert("step", Icon::new("󰻫", 0x839463));
    m.insert("stl", Icon::new("󰆧", 0x888888));
    m.insert("stp", Icon::new("󰻫", 0x839463));
    m.insert("strings", Icon::new("", 0x2596BE));
    m.insert("styl", Icon::new("", 0x8DC149));
    m.insert("sub", Icon::new("󰨖", 0xFFB713));
    m.insert("sublime", Icon::new("", 0xE37933));
    m.insert("suo", Icon::new("", 0x854CC7));
    m.insert("sv", Icon::new("󰍛", 0x019833));
    m.insert("svelte", Icon::new("", 0xFF3E00));
    m.insert("svg", Icon::new("󰜡", 0xFFB13B));
    m.insert("svh", Icon::new("󰍛", 0x019833));
    m.insert("swift", Icon::new("", 0xE37933));
    m.insert("t", Icon::new("", 0x519ABA));
    m.insert("tbc", Icon::new("󰛓", 0x1E5CB3));
    m.insert("tcl", Icon::new("󰛓", 0x1E5CB3));
    m.insert("templ", Icon::new("", 0xDBBD30));
    m.insert("terminal", Icon::new("", 0x31B53E));
    m.insert("test.js", Icon::new("", 0xCBCB41));
    m.insert("test.jsx", Icon::new("", 0x20C2E3));
    m.insert("test.ts", Icon::new("", 0x519ABA));
    m.insert("test.tsx", Icon::new("", 0x1354BF));
    m.insert("tex", Icon::new("", 0x3D6117));
    m.insert("tf", Icon::new("", 0x5F43E9));
    m.insert("tfvars", Icon::new("", 0x5F43E9));
    m.insert("tgz", Icon::new("", 0xECA517));
    m.insert("tmux", Icon::new("", 0x14BA19));
    m.insert("toml", Icon::new("", 0x9C4221));
    m.insert("torrent", Icon::new("", 0x44CDA8));
    m.insert("tres", Icon::new("", 0x6D8086));
    m.insert("ts", Icon::new("", 0x519ABA));
    m.insert("tscn", Icon::new("", 0x6D8086));
    m.insert("tsconfig", Icon::new("", 0xFF8700));
    m.insert("tsx", Icon::new("", 0x1354BF));
    m.insert("ttf", Icon::new("", 0xECECEC));
    m.insert("twig", Icon::new("", 0x8DC149));
    m.insert("txt", Icon::new("󰈙", 0x89E051));
    m.insert("txz", Icon::new("", 0xECA517));
    m.insert("typoscript", Icon::new("", 0xFF8700));
    m.insert("ui", Icon::new("", 0x015BF0));
    m.insert("v", Icon::new("󰍛", 0x019833));
    m.insert("vala", Icon::new("", 0x7239B3));
    m.insert("vh", Icon::new("󰍛", 0x019833));
    m.insert("vhd", Icon::new("󰍛", 0x019833));
    m.insert("vhdl", Icon::new("󰍛", 0x019833));
    m.insert("vim", Icon::new("", 0x019833));
    m.insert("vsh", Icon::new("", 0x5D87BF));
    m.insert("vsix", Icon::new("", 0x854CC7));
    m.insert("vue", Icon::new("", 0x8DC149));
    m.insert("wasm", Icon::new("", 0x5C4CDB));
    m.insert("wav", Icon::new("", 0x00AFFF));
    m.insert("webm", Icon::new("", 0xFD971F));
    m.insert("webmanifest", Icon::new("", 0xF1E05A));
    m.insert("webp", Icon::new("", 0xA074C4));
    m.insert("webpack", Icon::new("󰜫", 0x519ABA));
    m.insert("wma", Icon::new("", 0x00AFFF));
    m.insert("woff", Icon::new("", 0xECECEC));
    m.insert("woff2", Icon::new("", 0xECECEC));
    m.insert("wrl", Icon::new("󰆧", 0x888888));
    m.insert("wrz", Icon::new("󰆧", 0x888888));
    m.insert("wv", Icon::new("", 0x00AFFF));
    m.insert("wvc", Icon::new("", 0x00AFFF));
    m.insert("x", Icon::new("", 0x599EFF));
    m.insert("xaml", Icon::new("󰙳", 0x512BD4));
    m.insert("xcf", Icon::new("", 0x635B46));
    m.insert("xcplayground", Icon::new("", 0xE37933));
    m.insert("xcstrings", Icon::new("", 0x2596BE));
    m.insert("xls", Icon::new("󰈛", 0x207245));
    m.insert("xlsx", Icon::new("󰈛", 0x207245));
    m.insert("xm", Icon::new("", 0x519ABA));
    m.insert("xml", Icon::new("󰗀", 0xE37933));
    m.insert("xpi", Icon::new("", 0xFF1B01));
    m.insert("xul", Icon::new("", 0xE37933));
    m.insert("xz", Icon::new("", 0xECA517));
    m.insert("yaml", Icon::new("", 0x6D8086));
    m.insert("yml", Icon::new("", 0x6D8086));
    m.insert("zig", Icon::new("", 0xF69A1B));
    m.insert("zip", Icon::new("", 0xECA517));
    m.insert("zsh", Icon::new("", 0x89E051));
    m.insert("zst", Icon::new("", 0xECA517));
    m.insert("🔥", Icon::new("", 0xFF4C1F));
    m
});
