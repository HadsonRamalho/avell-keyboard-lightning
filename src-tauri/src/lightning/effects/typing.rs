use rdev::{listen, EventType};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::{
    sync::Mutex,
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::lightning::{hsv_to_rgb, update_led_color};

static TYPING_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct TypingHeatmapHandle {
    pub thread_handle: JoinHandle<()>,
    pub shutdown_tx: mpsc::Sender<()>,
}

pub static TYPING_HEATMAP_STATE: Mutex<Option<TypingHeatmapHandle>> = Mutex::new(None);

fn typing_heatmap_loop(shutdown_rx: Arc<Mutex<mpsc::Receiver<()>>>) -> std::io::Result<()> {
    let mut heat = 0.0;

    loop {
        if shutdown_rx.lock().unwrap().try_recv().is_ok() {
            break;
        }

        let count = TYPING_COUNTER.swap(0, Ordering::Relaxed);

        if count > 0 {
            heat += count as f32 * 0.06;
        } else {
            heat -= 0.06;
        }

        heat = heat.clamp(0.0, 1.0);

        let hue = 240.0 - (240.0 * heat);
        let (r, g, b) = hsv_to_rgb(hue, 1.0, heat.max(0.05));

        let _ = update_led_color(r, g, b);

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

use std::sync::Once;

static INIT_LISTENER: Once = Once::new();

fn ensure_global_listener() {
    INIT_LISTENER.call_once(|| {
        thread::spawn(|| {
            let _ = rdev::listen(move |event| {
                if let EventType::KeyPress(_) = event.event_type {
                    TYPING_COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            });
        });
    });
}

#[tauri::command]
pub fn start_typing_heatmap() -> Result<String, String> {
    stop_typing_heatmap().ok();

    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    let shutdown_rx = Arc::new(Mutex::new(shutdown_rx));

    let shutdown_rx_for_effect = Arc::clone(&shutdown_rx);

    ensure_global_listener();

    let thread_handle = thread::spawn(move || {
        let _ = typing_heatmap_loop(shutdown_rx_for_effect);
    });

    let mut state = TYPING_HEATMAP_STATE.lock().unwrap();
    *state = Some(TypingHeatmapHandle {
        thread_handle,
        shutdown_tx,
    });

    Ok("Typing heatmap started".into())
}

#[tauri::command]
pub fn stop_typing_heatmap() -> Result<String, String> {
    let mut state = TYPING_HEATMAP_STATE.lock().unwrap();

    if let Some(handle) = state.take() {
        let _ = handle.shutdown_tx.send(());

        if let Err(e) = handle.thread_handle.join() {
            return Err(format!("Erro ao parar efeito: {:?}", e));
        }

        Ok("Typing heatmap stopped".into())
    } else {
        Err("Typing heatmap not running".into())
    }
}

#[tauri::command]
pub fn is_typing_heatmap_active() -> bool {
    let state = TYPING_HEATMAP_STATE.lock().unwrap();
    state.is_some()
}
