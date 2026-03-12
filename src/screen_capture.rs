// Screen capture module - takes screenshots and calls Python OCR

use std::process::Command;
use std::path::Path;
use std::fs;

const SCREENSHOT_FILENAME: &str = "screenshot.png";
const PYTHON_SCRIPT: &str = "python_ocr/ocr_engine.py";

// Struct to manage screen capture
pub struct ScreenCapture {
    output_dir: String,
    screenshot_path: String,
    capture_count: u32,
}

impl ScreenCapture {
    pub fn new(output_dir: &str) -> Self {
        // Create output folder if it doesn't exist
        if !Path::new(output_dir).exists() {
            fs::create_dir_all(output_dir)
                .expect("Failed to create output directory");
        }

        let screenshot_path = format!("{}/{}", output_dir, SCREENSHOT_FILENAME);

        ScreenCapture {
            output_dir: output_dir.to_string(),
            screenshot_path,
            capture_count: 0,
        }
    }

    // Take a screenshot of the screen
    pub fn capture_screen(&mut self) -> Result<String, String> {
        println!("  Capturing screenshot...");

        let screens = screenshots::Screen::all()
            .map_err(|e| format!("Could not find screens: {}", e))?;

        if screens.is_empty() {
            return Err("No screens found".to_string());
        }

        let screen = &screens[0];

        let image = screen.capture()
            .map_err(|e| format!("Could not capture screen: {}", e))?;

        let save_path = format!("{}/screenshot_{}.png", self.output_dir, self.capture_count);

        image.save(&save_path)
            .map_err(|e| format!("Could not save screenshot: {}", e))?;

        self.screenshot_path = save_path.clone();
        self.capture_count += 1;

        println!("  Screenshot saved: {}", save_path);
        Ok(save_path)
    }

    // Call the Python OCR script on an image
    pub fn run_ocr(&self, image_path: &str) -> Result<String, String> {
        println!("  Running OCR on: {}", image_path);

        if !Path::new(PYTHON_SCRIPT).exists() {
            return Err(format!("Python script not found: {}", PYTHON_SCRIPT));
        }

        if !Path::new(image_path).exists() {
            return Err(format!("Image not found: {}", image_path));
        }

        // Run the python script and capture its output
        // Try "python" first, fallback to "python3" (common on Mac/Linux)
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
            }.map_err(|e| format!("Could not run Python or Python3: {}", e))?;

        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if text.is_empty() {
                Err("OCR returned no text".to_string())
            } else {
                println!("  OCR got {} characters", text.len());
                Ok(text)
            }
        } else {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            Err(format!("OCR failed: {}", err))
        }
    }

    // Take a screenshot and run OCR in one go
    #[allow(dead_code)]
    pub fn capture_and_ocr(&mut self) -> Result<String, String> {
        let path = self.capture_screen()?;
        self.run_ocr(&path)
    }

    #[allow(dead_code)]
    pub fn get_screenshot_path(&self) -> &str {
        &self.screenshot_path
    }

    #[allow(dead_code)]
    pub fn get_capture_count(&self) -> u32 {
        self.capture_count
    }

    #[allow(dead_code)]
    pub fn get_output_dir(&self) -> &str {
        &self.output_dir
    }
}
