use std::{collections::HashMap, io::stdin, path::Path};

use icons::Icon;

mod icons;

fn get_icon<'a>(
    path: &Path,
    icons_by_filename: &'a HashMap<&str, Icon>,
    icons_by_extension: &'a HashMap<&str, Icon>,
    default_icon: &'a Icon,
) -> (&'a str, (u8, u8, u8)) {
    if let Some(filename) = path.file_name() {
        if let Some(icon) = icons_by_filename.get(filename.to_str().unwrap()) {
            return (
                icon.icon,
                (icon.color_red, icon.color_green, icon.color_blue),
            );
        }
    }

    if let Some(extension) = path.extension() {
        if let Some(icon) = icons_by_extension.get(extension.to_str().unwrap()) {
            return (
                icon.icon,
                (icon.color_red, icon.color_green, icon.color_blue),
            );
        }
    }

    (
        default_icon.icon,
        (
            default_icon.color_red,
            default_icon.color_green,
            default_icon.color_blue,
        ),
    )
}

fn main() {
    let icons_by_filename = icons::ICONS_BY_FILENAME.lock().unwrap();
    let icons_by_extension = icons::ICONS_BY_FILE_EXTENSION.lock().unwrap();
    let default_icon = &icons::DEFAULT_ICON;

    // https://github.com/ibhagwan/fzf-lua/blob/2fa4913c7db0c22e02c003c6f09b7ebb2d0ed367/lua/fzf-lua/utils.lua#L40
    // Using the non-breaking space as a separator makes `fzf-lua` detect the
    // icon and the path as two separate columns.
    let non_breaking_space = '\u{2002}';

    let input = stdin();
    let mut buffer = String::new();
    while input.read_line(&mut buffer).unwrap() > 0 {
        let trimmed_lowercase = buffer.trim().to_lowercase();
        let path = Path::new(&trimmed_lowercase);
        let (icon, (r, g, b)) =
            get_icon(path, &icons_by_filename, &icons_by_extension, default_icon);

        print!("\x1b[38;2;{r};{g};{b}m{icon}\x1b[0m{non_breaking_space}{buffer}");

        buffer.clear();
    }
}
