use tauri::generate_context;

use crate::lightning::{
    is_screen_capture_active, start_screen_capture, stop_screen_capture, update_led_color,
};

mod lightning;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            update_keyboard_color,
            start_screen_capture,
            stop_screen_capture,
            is_screen_capture_active
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
