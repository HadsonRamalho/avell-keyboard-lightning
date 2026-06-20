use std::{
    env,
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

pub struct ObsidianTheme {
    pub status_bar_background: Option<String>,
}

pub fn update_obsidian_theme(theme_config: &ObsidianTheme) -> io::Result<()> {
    let css = format!(
        "body {{\n    --status-bar-background: {status_bar_background};\n}}",
        status_bar_background = theme_config
            .status_bar_background
            .as_deref()
            .unwrap_or("")
    );

    let home = env::var("HOME").map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
    let path = PathBuf::from(home)
        .join("Documentos/Obsidian Vault/.obsidian/snippets/headers.css");

    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    file.write_all(css.as_bytes())?;

    Ok(())
}
