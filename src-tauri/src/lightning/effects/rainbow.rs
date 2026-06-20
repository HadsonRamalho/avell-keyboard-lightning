use std::{
    io,
    sync::{mpsc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::lightning::{
    effects::screen_capture::SCREEN_CAPTURE_STATE,
    hsv_to_rgb,
    stop_all_effects, update_led_color,
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
        
        crate::lightning::theme_sync::sync_all_themes(r, g, b);

        hue += 2.0;
        if hue >= 360.0 {
            hue = 0.0;
        }

        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}
