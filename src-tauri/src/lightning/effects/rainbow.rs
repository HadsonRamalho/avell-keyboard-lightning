use std::{
    io,
    sync::{mpsc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::lightning::{
    brokers_frontend::update_brokers_color,
    discord::{update_discord_theme, DiscordTheme},
    effects::screen_capture::SCREEN_CAPTURE_STATE,
    hsv_to_rgb,
    obsidian::{update_obsidian_theme, ObsidianTheme},
    stop_all_effects, update_led_color,
    zed::{update_zed_theme, ZedTheme},
};

pub static RAINBOW_EFFECT_STATE: Mutex<Option<RainbowEffectHandle>> = Mutex::new(None);
pub struct RainbowEffectHandle {
    pub thread_handle: JoinHandle<()>,
    pub shutdown_tx: mpsc::Sender<()>,
}

#[tauri::command]
pub fn start_rainbow_effect() -> Result<String, String> {
    stop_all_effects();

    let mut rainbow_state = RAINBOW_EFFECT_STATE.lock().unwrap();

    if rainbow_state.is_some() {
        return Err("Rainbow effect is already running".to_string());
    }

    let mut screen_state = SCREEN_CAPTURE_STATE.lock().unwrap();
    if let Some(handle) = screen_state.take() {
        let _ = handle.shutdown_tx.send(());
        let _ = handle.thread_handle.join();
    }
    drop(screen_state);

    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let thread_handle = thread::spawn(move || {
        if let Err(e) = rainbow_effect_loop(shutdown_rx) {
            eprintln!("Rainbow effect error: {}", e);
        }
    });

    *rainbow_state = Some(RainbowEffectHandle {
        thread_handle,
        shutdown_tx,
    });

    Ok("Rainbow effect started".to_string())
}

#[tauri::command]
pub fn stop_rainbow_effect() -> Result<String, String> {
    let mut state = RAINBOW_EFFECT_STATE.lock().unwrap();

    if let Some(handle) = state.take() {
        let _ = handle.shutdown_tx.send(());

        if let Err(e) = handle.thread_handle.join() {
            return Err(format!("Error stopping rainbow effect: {:?}", e));
        }

        Ok("Rainbow effect stopped".to_string())
    } else {
        Err("Rainbow effect is not running".to_string())
    }
}

#[tauri::command]
pub fn is_rainbow_effect_active() -> bool {
    let state = RAINBOW_EFFECT_STATE.lock().unwrap();
    state.is_some()
}

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn rgb_to_string(r: u8, g: u8, b: u8) -> String {
    format!("{}, {}, {}", r, g, b)
}

pub fn darken_85(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let factor = 0.15;

    let dr = (r as f32 * factor).round() as u8;
    let dg = (g as f32 * factor).round() as u8;
    let db = (b as f32 * factor).round() as u8;

    (dr, dg, db)
}

pub fn darken_95(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let factor = 0.05;

    let dr = (r as f32 * factor).round() as u8;
    let dg = (g as f32 * factor).round() as u8;
    let db = (b as f32 * factor).round() as u8;

    (dr, dg, db)
}

fn rainbow_effect_loop(shutdown_rx: mpsc::Receiver<()>) -> io::Result<()> {
    let mut hue: f32 = 0.0;
    let saturation: f32 = 1.0;
    let value: f32 = 1.0;

    loop {
        if shutdown_rx.try_recv().is_ok() {
            break;
        }

        let (r, g, b) = hsv_to_rgb(hue, saturation, value);
        update_led_color(r, g, b)?;
        let rgb_color = rgb_to_string(r, g, b);

        let hex_color = rgb_to_hex(r, g, b);
        update_obsidian_theme(&ObsidianTheme {
            status_bar_background: Some(hex_color.clone()),
        })?;
        update_zed_theme(&ZedTheme {
            icon_accent: Some(hex_color.clone()),
            icon: Some(hex_color.clone()),
            string_primary: None,
            type_color: None,
            panel_indent_guide: Some(hex_color.clone()),
            scrollbar_thumb_background: Some(hex_color.clone()),
            scrollbar_track_border: Some(hex_color.clone()),
            editor_active_line_number: Some(hex_color.clone()),
            editor_indent_guide_active: Some(hex_color.clone()),
            ghost_element_active: Some(hex_color.clone()),
        })?;
        let rgb_color = rgb_to_string(r, g, b);

        let (r, g, b) = darken_85(r, g, b);
        let darker_rgb_color = rgb_to_string(r, g, b);
        let (r, g, b) = darken_95(r, g, b);
        let even_darker_rgb_color = rgb_to_string(r, g, b);
        update_discord_theme(&DiscordTheme {
            accent_color: rgb_color.clone(),
            accent_color2: rgb_color.clone(),
            link_color: rgb_color.clone(),

            text_brightest: rgb_color.clone(),
            text_brighter: rgb_color.clone(),
            text_bright: rgb_color.clone(),

            background_primary: darker_rgb_color.clone(),
            background_secondary: even_darker_rgb_color.clone(),
            background_tertiary: darker_rgb_color.clone(),
        })?;
        //update_brokers_color(hex_color.clone());

        hue += 2.0;
        if hue >= 360.0 {
            hue = 0.0;
        }

        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}
