use crate::lightning::{
    is_rainbow_effect_active, is_screen_capture_active, start_rainbow_effect, start_screen_capture,
    stop_rainbow_effect, stop_screen_capture, update_led_color,
};
use std::{fs, sync::Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager, RunEvent, WindowEvent,
};

mod lightning;

static SHOULD_EXIT: Mutex<bool> = Mutex::new(false);

fn is_supported_hardware() -> bool {
    let led_path = "/sys/class/leds/rgb:kbd_backlight/multi_intensity";
    if !std::path::Path::new(led_path).exists() {
        return false;
    }

    if let Ok(product_name) = fs::read_to_string("/sys/class/dmi/id/product_name") {
        let product_name = product_name.trim().to_lowercase();

        if product_name.contains("avell") || product_name.contains("storm 450r") {
            return true;
        }
    }

    if let Ok(sys_vendor) = fs::read_to_string("/sys/class/dmi/id/sys_vendor") {
        let sys_vendor = sys_vendor.trim().to_lowercase();

        if sys_vendor.contains("avell") {
            return true;
        }
    }

    false
}

#[tauri::command]
fn update_keyboard_color(red: u8, green: u8, blue: u8) -> Result<String, String> {
    match update_led_color(red, green, blue) {
        Ok(()) => Ok(format!(
            "Keyboard color updated to RGB({}, {}, {})",
            red, green, blue
        )),
        Err(e) => Err(format!("Failed to update keyboard color: {}", e)),
    }
}

#[tauri::command]
fn show_main_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
fn hide_main_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            update_keyboard_color,
            start_screen_capture,
            stop_screen_capture,
            is_screen_capture_active,
            start_rainbow_effect,
            stop_rainbow_effect,
            is_rainbow_effect_active,
            show_main_window,
            hide_main_window
        ])
        .setup(|app| {
            if !is_supported_hardware() {
                eprintln!("This application only works on Avell notebooks or Avell Storm 450r");
                std::process::exit(1);
            }

            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("Avell Keyboard Lightning")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        *SHOULD_EXIT.lock().unwrap() = true;
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = if window.is_visible().unwrap_or(false) {
                                window.hide()
                            } else {
                                window.show()
                            };
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                let should_exit = *SHOULD_EXIT.lock().unwrap();
                if !should_exit {
                    api.prevent_exit();
                }
            }
        });
}
