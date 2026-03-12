// This module takes screenshots of the computer screen and 
// sends them to the Python OCR engine to be read.

use std::process::Command;
use std::path::Path;
use std::fs;

const SCREENSHOT_FILENAME: &str = "screenshot.png";
const PYTHON_SCRIPT: &str = "python_ocr/ocr_engine.py";

pub struct ScreenCapture {
    output_dir: String,
    screenshot_path: String,
    capture_count: u32,
}

impl ScreenCapture {
    pub fn new(output_dir: &str) -> Self {
        // Create the output folder if it doesn't already exist.
        if !Path::new(output_dir).exists() {
            fs::create_dir_all(output_dir)
                .expect("Critical Error: Could not create the output directory.");
        }

        let screenshot_path = format!("{}/{}", output_dir, SCREENSHOT_FILENAME);

        ScreenCapture {
            output_dir: output_dir.to_string(),
            screenshot_path,
            capture_count: 0,
        }
    }

    // Capture the primary monitor and save it as a PNG file.
    pub fn capture_screen(&mut self) -> Result<String, String> {
        println!("  Capturing the current screen...");

        let screens = screenshots::Screen::all()
            .map_err(|e| format!("Could not detect any screens: {}", e))?;

        if screens.is_empty() {
            return Err("No computer monitor was found.".to_string());
        }

        // Capture the first (primary) monitor.
        let screen = &screens[0];

        let image = screen.capture()
            .map_err(|e| format!("Screen capture failed: {}", e))?;

        let save_path = format!("{}/screenshot_{}.png", self.output_dir, self.capture_count);

        image.save(&save_path)
            .map_err(|e| format!("Failed to save the image file: {}", e))?;

        self.screenshot_path = save_path.clone();
        self.capture_count += 1;

        println!("  Screen captured successfully: {}", save_path);
        Ok(save_path)
    }

    // Call the external Python script to read the numbers from the image.
    pub fn run_ocr(&self, image_path: &str) -> Result<String, String> {
        println!("  Analyzing image text: {}", image_path);

        if !Path::new(PYTHON_SCRIPT).exists() {
            return Err("The Python OCR script is missing from the directory.".to_string());
        }

        if !Path::new(image_path).exists() {
            return Err("The image file could not be found for analysis.".to_string());
        }

        // Run the python script and capture the text it returns.
        // We try "python" first, and if that doesn't work (like on Mac/Linux), we try "python3".
        let output = match Command::new("python")
            .arg(PYTHON_SCRIPT)
            .arg(image_path)
            .output() {
                Ok(out) => Ok(out),
                Err(_) => {
                    Command::new("python3")
                        .arg(PYTHON_SCRIPT)
                        .arg(image_path)
                        .output()
                }
            }.map_err(|e| format!("Error: Python is not installed or not working: {}", e))?;

        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if text.is_empty() {
                Err("The image analysis returned no text.".to_string())
            } else {
                println!("  Successfully read {} characters from the image.", text.len());
                Ok(text)
            }
        } else {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            Err(format!("Image analysis error: {}", err))
        }
    }
}
