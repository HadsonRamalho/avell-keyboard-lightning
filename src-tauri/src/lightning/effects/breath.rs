use std::{
    io,
    sync::{mpsc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::lightning::{
    effects::rainbow::rgb_to_hex,
    hsv_to_rgb,
    rgb_to_hsv, stop_all_effects, update_led_color,
};

pub static BREATH_EFFECT_STATE: Mutex<Option<BreathEffectHandle>> = Mutex::new(None);

pub struct BreathEffectHandle {
    pub thread_handle: JoinHandle<()>,
    pub shutdown_tx: mpsc::Sender<()>,
}

fn breath_effect_loop(
    shutdown_rx: mpsc::Receiver<()>,
    red: u8,
    green: u8,
    blue: u8,
) -> io::Result<()> {
    let (h, s, _) = rgb_to_hsv(red, green, blue);

    let mut value: f32 = 0.0;
    let mut increasing = true;

    loop {
        if shutdown_rx.try_recv().is_ok() {
            break;
        }

        let (r, g, b) = hsv_to_rgb(h, s, value);
        update_led_color(r, g, b)?;

        crate::lightning::theme_sync::sync_all_themes(r, g, b);

        if increasing {
            value += 0.06;
            if value >= 1.0 {
                value = 1.0;
                increasing = false;
            }
        } else {
            value -= 0.06;
            if value <= 0.06 {
                value = 0.06;
                increasing = true;
            }
        }

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

#[tauri::command]
pub fn start_breath_effect(red: u8, green: u8, blue: u8) -> Result<String, String> {
    stop_all_effects();

    let mut breath_state = BREATH_EFFECT_STATE.lock().unwrap();

    if breath_state.is_some() {
        return Err("Breath effect is already running".to_string());
    }

    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let thread_handle = thread::spawn(move || {
        if let Err(e) = breath_effect_loop(shutdown_rx, red, green, blue) {
            eprintln!("Breath effect error: {}", e);
        }
    });

    *breath_state = Some(BreathEffectHandle {
        thread_handle,
        shutdown_tx,
    });

    Ok("Breath effect started".to_string())
}

#[tauri::command]
pub fn stop_breath_effect() -> Result<String, String> {
    let mut state = BREATH_EFFECT_STATE.lock().unwrap();

    if let Some(handle) = state.take() {
        let _ = handle.shutdown_tx.send(());

        if let Err(e) = handle.thread_handle.join() {
            return Err(format!("Error stopping breath effect: {:?}", e));
        }

        Ok("Breath effect stopped".to_string())
    } else {
        Err("Breath effect is not running".to_string())
    }
}

#[tauri::command]
pub fn is_breath_effect_active() -> bool {
    let state = BREATH_EFFECT_STATE.lock().unwrap();
    state.is_some()
}
