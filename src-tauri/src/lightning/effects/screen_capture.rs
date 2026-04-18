use scrap::{Capturer, Display};
use std::{
    io::{self, ErrorKind},
    sync::{mpsc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::lightning::{
    effects::rainbow::rgb_to_hex,
    hsv_to_rgb, rgb_to_hsv, stop_all_effects, update_led_color,
    zed::{update_zed_theme, ZedTheme},
};

pub static SCREEN_CAPTURE_STATE: Mutex<Option<ScreenCaptureHandle>> = Mutex::new(None);

pub struct ScreenCaptureHandle {
    pub thread_handle: JoinHandle<()>,
    pub shutdown_tx: mpsc::Sender<()>,
}

#[tauri::command]
pub fn start_screen_capture() -> Result<String, String> {
    stop_all_effects();

    let mut state = SCREEN_CAPTURE_STATE.lock().unwrap();

    if state.is_some() {
        return Err("Screen capture is already running".to_string());
    }

    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let thread_handle = thread::spawn(move || {
        if let Err(e) = update_color_using_screen(shutdown_rx) {
            eprintln!("Screen capture error: {}", e);
        }
    });

    *state = Some(ScreenCaptureHandle {
        thread_handle,
        shutdown_tx,
    });

    Ok("Screen capture started".to_string())
}

#[tauri::command]
pub fn stop_screen_capture() -> Result<String, String> {
    let mut state = SCREEN_CAPTURE_STATE.lock().unwrap();

    if let Some(handle) = state.take() {
        let _ = handle.shutdown_tx.send(());

        if let Err(e) = handle.thread_handle.join() {
            return Err(format!("Error stopping screen capture: {:?}", e));
        }

        Ok("Screen capture stopped".to_string())
    } else {
        Err("Screen capture is not running".to_string())
    }
}

#[tauri::command]
pub fn is_screen_capture_active() -> bool {
    let state = SCREEN_CAPTURE_STATE.lock().unwrap();
    state.is_some()
}

fn update_color_using_screen(shutdown_rx: mpsc::Receiver<()>) -> io::Result<()> {
    let display = Display::primary().map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    let mut capturer = Capturer::new(display).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    loop {
        if shutdown_rx.try_recv().is_ok() {
            break;
        }

        let frame = loop {
            match capturer.frame() {
                Ok(buffer) => break buffer.to_vec(),
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                Err(e) => {
                    return Err(io::Error::new(
                        ErrorKind::Other,
                        format!("Erro na captura: {}", e),
                    ))
                }
            }
        };

        let width = capturer.width();
        let height = capturer.height();

        let mut r_total: u64 = 0;
        let mut g_total: u64 = 0;
        let mut b_total: u64 = 0;
        let mut count: u64 = 0;

        for y in 0..height {
            for x in 0..width {
                let i = (y * width + x) * 4;
                let b = frame[i] as u64;
                let g = frame[i + 1] as u64;
                let r = frame[i + 2] as u64;

                r_total += r;
                g_total += g;
                b_total += b;
                count += 1;
            }
        }

        let avg_r = (r_total / count) as u8;
        let avg_g = (g_total / count) as u8;
        let avg_b = (b_total / count) as u8;

        let (h, mut s, v) = rgb_to_hsv(avg_r, avg_g, avg_b);
        s = (s * 1.85).min(1.0);
        let (r_final, g_final, b_final) = hsv_to_rgb(h, s, v);

        update_led_color(r_final, g_final, b_final)?;

        let hex_color = rgb_to_hex(r_final, g_final, b_final);
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

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
