use std::{env, path::PathBuf, sync::Mutex};
use serde::{Deserialize, Serialize};

use crate::lightning::{
    discord::{update_discord_theme, DiscordTheme},
    obsidian::{update_obsidian_theme, ObsidianTheme},
    zed::{update_zed_theme, ZedTheme},
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModuleStatus {
    pub id: String,
    pub name: String,
    pub available: bool,
    pub enabled: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModulesState {
    pub discord: bool,
    pub obsidian: bool,
    pub zed: bool,
}

impl Default for ModulesState {
    fn default() -> Self {
        Self {
            discord: true,
            obsidian: true,
            zed: true,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref MODULES_STATE: Mutex<ModulesState> = Mutex::new(ModulesState::default());
}

pub fn is_discord_available() -> bool {
    env::var("HOME")
        .map(|h| PathBuf::from(&h).join(".config/legcord").exists() || PathBuf::from(&h).join(".config/Legcord").exists())
        .unwrap_or(false)
}

pub fn is_obsidian_available() -> bool {
    env::var("HOME")
        .map(|h| PathBuf::from(h).join("Documentos/Obsidian Vault/.obsidian").exists())
        .unwrap_or(false)
}

pub fn is_zed_available() -> bool {
    env::var("HOME")
        .map(|h| PathBuf::from(h).join(".config/zed").exists())
        .unwrap_or(false)
}

#[tauri::command]
pub fn get_modules_status() -> Vec<ModuleStatus> {
    let state = MODULES_STATE.lock().unwrap();
    vec![
        ModuleStatus {
            id: "zed".into(),
            name: "Zed Editor".into(),
            available: is_zed_available(),
            enabled: state.zed,
        },
        ModuleStatus {
            id: "discord".into(),
            name: "Discord (Legcord)".into(),
            available: is_discord_available(),
            enabled: state.discord,
        },
        ModuleStatus {
            id: "obsidian".into(),
            name: "Obsidian".into(),
            available: is_obsidian_available(),
            enabled: state.obsidian,
        },
    ]
}

#[tauri::command]
pub fn set_module_enabled(id: String, enabled: bool) {
    let mut state = MODULES_STATE.lock().unwrap();
    match id.as_str() {
        "zed" => state.zed = enabled,
        "discord" => state.discord = enabled,
        "obsidian" => state.obsidian = enabled,
        _ => {}
    }
}

pub fn darken_color(r: u8, g: u8, b: u8, factor: f32) -> (u8, u8, u8) {
    let r = (r as f32 * factor).round() as u8;
    let g = (g as f32 * factor).round() as u8;
    let b = (b as f32 * factor).round() as u8;
    (r, g, b)
}

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

pub fn rgb_to_string(r: u8, g: u8, b: u8) -> String {
    format!("{}, {}, {}", r, g, b)
}

pub fn sync_all_themes(r: u8, g: u8, b: u8) {
    let state = MODULES_STATE.lock().unwrap().clone();
    let hex_color = rgb_to_hex(r, g, b);

    if state.zed && is_zed_available() {
        let _ = update_zed_theme(&ZedTheme::from_accent(hex_color.clone()));
    }

    if state.discord && is_discord_available() {
        let rgb_str = rgb_to_string(r, g, b);
        
        let (r85, g85, b85) = darken_color(r, g, b, 0.15);
        let darker_rgb = rgb_to_string(r85, g85, b85);
        
        let (r95, g95, b95) = darken_color(r, g, b, 0.05);
        let even_darker_rgb = rgb_to_string(r95, g95, b95);
        
        let _ = update_discord_theme(&DiscordTheme {
            accent_color: rgb_str.clone(),
            accent_color2: rgb_str.clone(),
            link_color: rgb_str.clone(),
            background_primary: darker_rgb.clone(),
            background_secondary: even_darker_rgb,
            background_tertiary: darker_rgb,
        });
    }

    if state.obsidian && is_obsidian_available() {
        let _ = update_obsidian_theme(&ObsidianTheme {
            status_bar_background: Some(hex_color.clone()),
        });
    }
}
