use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

use crate::lightning::effects::screen_capture::SCREEN_CAPTURE_STATE;
use crate::lightning::effects::typing::TYPING_HEATMAP_STATE;
use crate::lightning::effects::breath::BREATH_EFFECT_STATE;
use crate::lightning::effects::rainbow::RAINBOW_EFFECT_STATE;

pub mod discord;
pub mod effects;
pub mod obsidian;
pub mod zed;
pub mod theme_sync;

use crate::lightning::theme_sync::sync_all_themes;

#[tauri::command]
pub fn update_led_color(red: u8, green: u8, blue: u8) -> io::Result<()> {
    let path = Path::new("/sys/class/leds/rgb:kbd_backlight/multi_intensity");

    let mut file = OpenOptions::new().write(true).open(&path)?;

    let data = format!("{} {} {}", red, green, blue);

    file.write_all(data.as_bytes())?;

    sync_all_themes(red, green, blue);

    Ok(())
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    (h, s, v)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = match h {
        h if (0.0..60.0).contains(&h) => (c, x, 0.0),
        h if (60.0..120.0).contains(&h) => (x, c, 0.0),
        h if (120.0..180.0).contains(&h) => (0.0, c, x),
        h if (180.0..240.0).contains(&h) => (0.0, x, c),
        h if (240.0..300.0).contains(&h) => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;

    (r, g, b)
}

pub fn stop_all_effects() {
    {
        let mut screen_state = SCREEN_CAPTURE_STATE.lock().unwrap();
        if let Some(handle) = screen_state.take() {
            let _ = handle.shutdown_tx.send(());
            let _ = handle.thread_handle.join();
        }
    }

    {
        let mut rainbow_state = RAINBOW_EFFECT_STATE.lock().unwrap();
        if let Some(handle) = rainbow_state.take() {
            let _ = handle.shutdown_tx.send(());
            let _ = handle.thread_handle.join();
        }
    }

    {
        let mut breath_state = BREATH_EFFECT_STATE.lock().unwrap();
        if let Some(handle) = breath_state.take() {
            let _ = handle.shutdown_tx.send(());
            let _ = handle.thread_handle.join();
        }
    }

    {
        let mut typing_state = TYPING_HEATMAP_STATE.lock().unwrap();
        if let Some(handle) = typing_state.take() {
            let _ = handle.shutdown_tx.send(());
            let _ = handle.thread_handle.join();
        }
    }
}
