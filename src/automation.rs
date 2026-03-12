// Automation module - uses Enigo to simulate keyboard and mouse

use enigo::{
    Enigo, Keyboard, Mouse, Settings,
    Direction::{Click, Press, Release},
    {Coordinate::Abs, Button, Key},
};
use arboard::Clipboard;
use std::thread;
use std::time::Duration;

// Enum for the different types of actions we can do
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AutomationAction {
    TypeText(String),
    MoveMouse(i32, i32),
    ClickMouse(i32, i32),
    KeyPress(String),
}

// Struct that holds the Enigo instance and a log of actions
pub struct Automator {
    enigo: Enigo,
    clipboard: Option<Clipboard>,
    action_log: Vec<AutomationAction>,
}

impl Automator {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default())
            .expect("Failed to start Enigo");
        let clipboard = Clipboard::new().ok();
        let action_log: Vec<AutomationAction> = Vec::new();
        Automator { enigo, clipboard, action_log }
    }

    // Type text using keyboard simulation with a human-like delay
    pub fn type_text(&mut self, text: &str) {
        println!("  Typing: \"{}\"", text.trim());
        
        for c in text.chars() {
            match self.enigo.text(&c.to_string()) {
                Ok(_) => {
                    thread::sleep(Duration::from_millis(30));
                }
                Err(e) => println!("  Failed to type character: {}", e),
            }
        }
        
        self.action_log.push(AutomationAction::TypeText(text.to_string()));
    }

    // Press a single key
    pub fn key_click(&mut self, key: Key) {
        let _ = self.enigo.key(key, Click);
        thread::sleep(Duration::from_millis(100));
    }

    // Perform a combination like Ctrl+C
    pub fn key_combination(&mut self, modifier: Key, key: Key) {
        let _ = self.enigo.key(modifier, Press);
        thread::sleep(Duration::from_millis(50));
        let _ = self.enigo.key(key, Click);
        thread::sleep(Duration::from_millis(50));
        let _ = self.enigo.key(modifier, Release);
        thread::sleep(Duration::from_millis(100));
    }

    // Clear clipboard
    pub fn clear_clipboard(&mut self) {
        if let Some(ref mut cb) = self.clipboard {
            let _ = cb.set_text("");
        }
    }

    // Get text from clipboard (Alias for compatibility)
    pub fn get_clipboard_text(&mut self) -> String {
        self.get_clipboard()
    }

    // Get text from clipboard
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

    // Move the mouse to a position
    #[allow(dead_code)]
    pub fn move_mouse(&mut self, x: i32, y: i32) {
        println!("  Moving mouse to ({}, {})", x, y);
        match self.enigo.move_mouse(x, y, Abs) {
            Ok(_) => {}
            Err(e) => println!("  Failed to move mouse: {}", e),
        }
        self.action_log.push(AutomationAction::MoveMouse(x, y));
    }

    // Click at a position
    #[allow(dead_code)]
    pub fn click_at(&mut self, x: i32, y: i32) {
        println!("  Clicking at ({}, {})", x, y);
        let _ = self.enigo.move_mouse(x, y, Abs);
        match self.enigo.button(Button::Left, Click) {
            Ok(_) => {}
            Err(e) => println!("  Failed to click: {}", e),
        }
        self.action_log.push(AutomationAction::ClickMouse(x, y));
    }

    // Borrow the action log (read-only reference)
    #[allow(dead_code)]
    pub fn get_action_log(&self) -> &Vec<AutomationAction> {
        &self.action_log
    }

    // Print what actions were performed
    #[allow(dead_code)]
    pub fn print_action_log(&self) {
        println!("\nAction Log:");

        if self.action_log.is_empty() {
            println!("  No actions yet.");
            return;
        }

        for (i, action) in self.action_log.iter().enumerate() {
            let desc = match action {
                AutomationAction::TypeText(text) => format!("Typed: \"{}\"", text.trim()),
                AutomationAction::MoveMouse(x, y) => format!("Moved mouse to ({}, {})", x, y),
                AutomationAction::ClickMouse(x, y) => format!("Clicked at ({}, {})", x, y),
                AutomationAction::KeyPress(key) => format!("Pressed: {}", key),
            };
            println!("  {}. {}", i + 1, desc);
        }
    }

}
