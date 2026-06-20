use std::{
    env,
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

fn get_quickcss_path() -> io::Result<PathBuf> {
    let home = env::var("HOME").map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
    let path = PathBuf::from(home).join(".config/legcord/quickCss.css");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    Ok(path)
}

pub struct DiscordTheme {
    pub accent_color: String,
    pub accent_color2: String,
    pub link_color: String,
    pub background_primary: String,
    pub background_secondary: String,
    pub background_tertiary: String,
}

pub fn update_discord_theme(theme_config: &DiscordTheme) -> io::Result<()> {
    let css = format!(
        "@import url(\"https://mwittrien.github.io/BetterDiscordAddons/Themes/DiscordRecolor/DiscordRecolor.css\");\n\
        \n\
        :root {{\n\
            --accentcolor: {accent_color};\n\
            --accentcolor2: {accent_color2};\n\
            --linkcolor: {link_color};\n\
            --font: gg sans;\n\
            --backgroundprimary: {background_primary};\n\
            --backgroundsecondary: {background_secondary};\n\
            --backgroundtertiary: {background_tertiary};\n\
        }}",
        accent_color = theme_config.accent_color,
        accent_color2 = theme_config.accent_color2,
        link_color = theme_config.link_color,
        background_primary = theme_config.background_primary,
        background_secondary = theme_config.background_secondary,
        background_tertiary = theme_config.background_tertiary,
    );

    let path = get_quickcss_path()?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    file.write_all(css.as_bytes())?;

    Ok(())
}
