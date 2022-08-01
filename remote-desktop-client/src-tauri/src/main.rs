#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

use commands::{generate_account, key_event, mouse_event};
use enigo::Key;
mod commands;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref KEYMAP: HashMap<&'static str, Key> = {
        let mut m = HashMap::new();
        m.insert("Control", Key::Control);
        m.insert("Alt", Key::Alt);
        m.insert("Delete", Key::Delete);
        m.insert("Escape", Key::Escape);
        m.insert("End", Key::End);
        m.insert("Home", Key::Home);
        m.insert(" ", Key::Space);
        m.insert("Shift", Key::Shift);
        m.insert("Tab", Key::Tab);
        m.insert("PageUp", Key::PageUp);
        m.insert("PageDown", Key::PageDown);
        m.insert("Backspace", Key::Backspace);
        m.insert("CapsLock", Key::CapsLock);
        m.insert("Meta", Key::Meta);

        m.insert("F1", Key::F1);
        m.insert("F2", Key::F2);
        m.insert("F3", Key::F3);
        m.insert("F4", Key::F4);
        m.insert("F5", Key::F5);
        m.insert("F6", Key::F6);
        m.insert("F7", Key::F7);
        m.insert("F8", Key::F8);
        m.insert("F9", Key::F9);
        m.insert("F10", Key::F10);
        m.insert("F11", Key::F11);
        m.insert("F12", Key::F12);

        m
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            key_event,
            mouse_event,
            generate_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
