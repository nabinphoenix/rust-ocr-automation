// This module handles standard computer interactions using the keyboard and mouse.
// It is used to simulate typing or clicking on the screen.

use enigo::{
    Enigo, Keyboard, Mouse, Settings,
    Direction::{Click, Press, Release},
    {Coordinate::Abs, Button, Key},
};
use arboard::Clipboard;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AutomationAction {
    TypeText(String),
    MoveMouse(i32, i32),
    ClickMouse(i32, i32),
    KeyPress(String),
}

pub struct Automator {
    enigo: Enigo,
    clipboard: Option<Clipboard>,
    action_log: Vec<AutomationAction>,
}

impl Automator {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default())
            .expect("Could not initialize the automation controller.");
        let clipboard = Clipboard::new().ok();
        let action_log: Vec<AutomationAction> = Vec::new();
        Automator { enigo, clipboard, action_log }
    }

    // Types out a string of text with a natural-looking delay between keys.
    pub fn type_text(&mut self, text: &str) {
        println!("  Typing text: \"{}\"", text.trim());
        
        for c in text.chars() {
            match self.enigo.text(&c.to_string()) {
                Ok(_) => {
                    // Small delay to prevent typing errors.
                    thread::sleep(Duration::from_millis(30));
                }
                Err(e) => println!("  Typing Error: {}", e),
            }
        }
        
        self.action_log.push(AutomationAction::TypeText(text.to_string()));
    }

    // Single key press and release.
    pub fn key_click(&mut self, key: Key) {
        let _ = self.enigo.key(key, Click);
        thread::sleep(Duration::from_millis(100));
    }

    // Simulates holding a modifier key while pressing another key (like Ctrl+C).
    pub fn key_combination(&mut self, modifier: Key, key: Key) {
        let _ = self.enigo.key(modifier, Press);
        thread::sleep(Duration::from_millis(50));
        let _ = self.enigo.key(key, Click);
        thread::sleep(Duration::from_millis(50));
        let _ = self.enigo.key(modifier, Release);
        thread::sleep(Duration::from_millis(100));
    }

    pub fn get_clipboard(&mut self) -> String {
        if let Some(ref mut cb) = self.clipboard {
            match cb.get_text() {
                Ok(text) => text.trim().to_string(),
                Err(_) => String::new(),
            }
        } else {
            String::new()
        }
    }

    // Moves the mouse pointer to a specific coordinate on the screen.
    #[allow(dead_code)]
    pub fn move_mouse(&mut self, x: i32, y: i32) {
        println!("  Moving mouse to position ({}, {})", x, y);
        match self.enigo.move_mouse(x, y, Abs) {
            Ok(_) => {}
            Err(e) => println!("  Mouse Movement Error: {}", e),
        }
        self.action_log.push(AutomationAction::MoveMouse(x, y));
    }

    // Moves the mouse and performs a left-click.
    #[allow(dead_code)]
    pub fn click_at(&mut self, x: i32, y: i32) {
        println!("  Clicking at ({}, {})", x, y);
        let _ = self.enigo.move_mouse(x, y, Abs);
        match self.enigo.button(Button::Left, Click) {
            Ok(_) => {}
            Err(e) => println!("  Click Error: {}", e),
        }
        self.action_log.push(AutomationAction::ClickMouse(x, y));
    }
}
