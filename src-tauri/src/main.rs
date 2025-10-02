// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lightning;

fn main() {
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    avell_keyboard_lightning_lib::run()
}
