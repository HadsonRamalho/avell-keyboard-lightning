use scrap::{Capturer, Display};
use std::sync::mpsc;
use std::{
    fs::OpenOptions,
    io::{self, ErrorKind, Write},
    path::Path,
    sync::Mutex,
    thread::{self, JoinHandle},
    time::Duration,
};

pub static SCREEN_CAPTURE_STATE: Mutex<Option<ScreenCaptureHandle>> = Mutex::new(None);

pub struct ScreenCaptureHandle {
    pub thread_handle: JoinHandle<()>,
    pub shutdown_tx: mpsc::Sender<()>,
}

#[tauri::command]
pub fn update_led_color(red: u8, green: u8, blue: u8) -> io::Result<()> {
    let path = Path::new("/sys/class/leds/rgb:kbd_backlight/multi_intensity");

    let mut file = OpenOptions::new().write(true).open(&path)?;

    let data = format!("{} {} {}", red, green, blue);

    file.write_all(data.as_bytes())?;

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

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

#[tauri::command]
pub fn start_screen_capture() -> Result<String, String> {
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
