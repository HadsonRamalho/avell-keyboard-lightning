use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

fn get_quickcss() -> String {
    let path = "/home/hadson/.config/legcord/quickCss.css".to_string();
    path
}

pub struct DiscordTheme {
    pub accent_color: String,
    pub accent_color2: String,
    pub link_color: String,
    pub text_brightest: String,
    pub text_brighter: String,
    pub text_bright: String,
    pub background_primary: String,
    pub background_secondary: String,
    pub background_tertiary: String,
}

pub fn update_discord_theme(theme_config: &DiscordTheme) -> io::Result<()> {
    let json = format!(
        "
        @import url(\"https://mwittrien.github.io/BetterDiscordAddons/Themes/DiscordRecolor/DiscordRecolor.css\");

        :root {{
            --accentcolor: {accent_color};
            --accentcolor2: {accent_color2};
            --linkcolor: {link_color};

            --font: gg sans;

            --backgroundprimary: {background_primary};
            --backgroundsecondary: {background_secondary};
            --backgroundtertiary: {background_tertiary};

        }}",
        accent_color = theme_config.accent_color.clone(),
        accent_color2 = theme_config.accent_color2.clone(),
        link_color = theme_config.link_color.clone(),

        background_primary = theme_config.background_primary.clone(),
        background_secondary = theme_config.background_secondary.clone(),
        background_tertiary = theme_config.background_tertiary.clone(),
    );

    let path = get_quickcss();

    let mut file = OpenOptions::new().write(true).open(&path)?;

    file.write_all(json.as_bytes())?;

    Ok(())
}
