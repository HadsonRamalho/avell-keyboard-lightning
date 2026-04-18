use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

pub struct ObsidianTheme {
    pub status_bar_background: Option<String>,
}

pub fn update_obsidian_theme(theme_config: &ObsidianTheme) -> io::Result<()> {
    let json = format!(
        "body {{
            --status-bar-background: {status_bar_background};
        }}",
        status_bar_background = theme_config
            .status_bar_background
            .clone()
            .unwrap_or("".to_string())
    );

    let path = Path::new("/home/hadson/Documentos/Obsidian Vault/.obsidian/snippets/headers.css");

    let mut file = OpenOptions::new().write(true).open(&path)?;

    file.write_all(json.as_bytes())?;

    Ok(())
}
