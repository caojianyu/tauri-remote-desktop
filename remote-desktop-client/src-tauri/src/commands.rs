use serde::Serialize;
use tauri::command;

use enigo::{Enigo, MouseButton, MouseControllable};
use enigo::{Key, KeyboardControllable};
use uuid::Uuid;

use rand::Rng;

use crate::KEYMAP;

#[derive(Serialize)]
pub struct Account {
    id: String,
    password: String,
}

#[command]
pub fn generate_account() -> Account {
    let uuid = Uuid::new_v4();
    let id = uuid.to_string().replace("-", "");

    let mut password = String::new();

    let mut random = rand::thread_rng();

    let mut i = 0;
    while i < 8 {
        let num = random.gen_range(0..10);
        password.push_str(&num.to_string());
        i += 1;
    }

    Account { id, password }
}

#[command]
pub fn mouse_event(x: i32, y: i32, event_type: &str) {
    let mut enigo = Enigo::new();

    println!("type {}", event_type);
    match event_type {
        "left-mouse-down" => {
            enigo.mouse_down(MouseButton::Left);
        }
        "left-mouse-up" => {
            enigo.mouse_up(MouseButton::Left);
        }
        "right-mouse-down" => {
            enigo.mouse_down(MouseButton::Right);
        }
        "right-mouse-up" => {
            enigo.mouse_down(MouseButton::Right);
        }
        "left-click" => {
            enigo.mouse_click(MouseButton::Left);
        }
        "right-click" => {
            enigo.mouse_click(MouseButton::Right);
        }
        "mouse-move" => {
            enigo.mouse_move_to(x, y);
        }
        "wheel-up" => {
            enigo.mouse_scroll_y(2);
        }
        "wheel-down" => {
            enigo.mouse_scroll_y(-2);
        }
        _ => {}
    }
}

#[command]
pub fn key_event(event_type: &str, key: &str) {
    let mut enigo = Enigo::new();

    let k: Key;

    if key.len() > 1 {
        // k = *KEYMAP.get(key).unwrap();
        match KEYMAP.get(key) {
            Some(val) => k = *val,
            None => {
                println!("get key error by map");
                return;
            }
        }
    } else {
        let c: Vec<char> = key.chars().collect();
        k = Key::Layout(c[0]);
    }

    match event_type {
        "key-down" => {
            enigo.key_down(k);
        }
        "key-up" => {
            enigo.key_up(k);
        }
        "key-click" => {
            enigo.key_click(k);
        }
        _ => {}
    }
}
