// This is the main controller of the application.
// It handles user input from the menu and connects all the different parts like OCR, 
// math evaluation, and saving results.

mod automation;
mod screen_capture;
mod expression_detector;
mod expression_evaluator;
mod result_manager;

use screen_capture::ScreenCapture;
use expression_detector::ExpressionDetector;
use expression_evaluator::ExpressionEvaluator;
use result_manager::{ResultManager, OutputFormat};
use serde::Deserialize;

use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

const OUTPUT_DIR: &str = "output";
const RESULTS_FILE: &str = "output/results.txt";

// Different ways a user can provide math problems to the system.
#[derive(Debug, Clone, PartialEq)]
enum InputMethod {
    ImagePath,
    Screenshot,
    TextFile,
    Exit,
}

// Data structure to hold the JSON response coming from the Python OCR script.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct OcrResponse {
    raw_ocr: String,
    results: Vec<OcrResultEntry>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct OcrResultEntry {
    expression: String,
    value: String,
}

// Simple helper to print a prompt and get a trimmed string from the user.
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Cleanup: remove extra spaces and quotes if the user copied a path.
    input.trim().trim_matches('"').trim().to_string()
}

// Display the main menu and get the user's choice.
fn show_menu() -> InputMethod {
    println!("\n--- Select Input Method ---");
    println!("  1. Enter Image Path  (Process a saved image)");
    println!("  2. Take Screenshot   (Capture and solve from screen)");
    println!("  3. Enter File Path   (Read equations from a text file)");
    println!("  0. Exit              (Close the application)");

    loop {
        let choice = read_input("\nEnter your choice (1/2/3/0): ");

        match choice.as_str() {
            "1" => return InputMethod::ImagePath,
            "2" => return InputMethod::Screenshot,
            "3" => return InputMethod::TextFile,
            "0" | "exit" | "quit" => return InputMethod::Exit,
            _ => println!("Invalid choice. Please pick a number from the menu."),
        }
    }
}

// Ask for a file path and verify it actually exists on the computer.
fn get_image_path() -> String {
    loop {
        let path = read_input("Enter the image path: ");

        if Path::new(&path).exists() {
            let lower = path.to_lowercase();
            // Basic check for image extensions.
            if lower.ends_with(".png") || lower.ends_with(".jpg")
                || lower.ends_with(".jpeg") || lower.ends_with(".bmp")
                || lower.ends_with(".tiff")
            {
                return path;
            } else {
                println!("Warning: This file might not be a valid image format.");
                let confirm = read_input("Try to process it anyway? (y/n): ");
                if confirm.to_lowercase() == "y" {
                    return path;
                }
            }
        } else {
            println!("File not found. Please check the path and try again: {}", path);
        }
    }
}

// Get the path to a text file for reading equations directly.
fn get_text_file_path() -> String {
    loop {
        let path = read_input("Enter the text file path: ");

        if Path::new(&path).exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    if content.is_empty() {
                        println!("This file is empty. Please pick a file with math problems.");
                    } else {
                        println!("File loaded successfully: {} ({} chars)", path, content.len());
                        return path;
                    }
                }
                Err(e) => {
                    println!("Could not read the file: {}", e);
                }
            }
        } else {
            println!("File not found: {}", path);
        }
    }
}

// Fallback text used only if the OCR process fails completely.
fn get_sample_text() -> String {
    String::from(
        "Sample Math List:\n\
         (500 * 2) / 5\n\
         123 + 456 - 78\n"
    )
}

fn main() {
    println!("\n=== Screen-Aware Math Automation System (for Treeleaf AI) ===\n");

    let input_method = show_menu();

    // End program if user chooses exit.
    if input_method == InputMethod::Exit {
        println!("Exiting program. Goodbye!\n");
        return;
    }

    let mut target_file: Option<String> = None;
    let mut expressions: Vec<String> = Vec::new();
    let mut source_name = String::from("Unknown Source");

    println!("\n--- Initializing Process ---");

    // Capture text based on the user's choice.
    let ocr_text: String = match input_method {
        InputMethod::ImagePath | InputMethod::Screenshot => {
            let path = if input_method == InputMethod::ImagePath {
                let p = get_image_path();
                source_name = Path::new(&p).file_name().and_then(|s| s.to_str()).unwrap_or(&p).to_string();
                p
            } else {
                // Screenshot countdown gives the user time to switch to the right window.
                println!("\nPreparing to capture screen...");
                for i in (1..=5).rev() {
                    print!("Capturing in {}... ", i);
                    io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
                println!("Capturing now!\n");
                let mut screen = ScreenCapture::new(OUTPUT_DIR);
                let p = screen.capture_screen().unwrap_or_default();
                source_name = format!("Screenshot ({})", Path::new(&p).file_name().and_then(|s| s.to_str()).unwrap_or(&p));
                p
            };

            if path.is_empty() {
                get_sample_text()
            } else {
                let screen = ScreenCapture::new(OUTPUT_DIR);
                match screen.run_ocr(&path) {
                    Ok(json_str) => {
                        // Parse JSON output from the Python script.
                        match serde_json::from_str::<OcrResponse>(&json_str) {
                            Ok(resp) => {
                                println!("OCR completed successfully!");
                                let mut seen = std::collections::HashSet::new();
                                for entry in &resp.results {
                                    let clean = entry.expression.replace(" ", "");
                                    if !entry.expression.contains("Error") && !seen.contains(&clean) {
                                        expressions.push(entry.expression.clone());
                                        seen.insert(clean);
                                    }
                                }
                                println!("  System found {} math problems in the image.", expressions.len());
                                resp.raw_ocr
                            }
                            Err(_) => {
                                // If Python returned plain text instead of JSON.
                                json_str
                            }
                        }
                    }
                    Err(e) => {
                        println!("OCR Process failed: {}. Falling back to sample text.", e);
                        get_sample_text()
                    }
                }
            }
        }

        InputMethod::TextFile => {
            let file_path = get_text_file_path();
            source_name = Path::new(&file_path).file_name().and_then(|s| s.to_str()).unwrap_or(&file_path).to_string();
            target_file = Some(file_path.clone());
            println!("Reading equations from: {}", file_path);

            match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(e) => {
                    println!("Failed to read text file: {}.", e);
                    get_sample_text()
                }
            }
        }
        
        _ => get_sample_text(),
    };

    // If reading from a text file, we don't want to re-process old results.
    let cleaned_ocr_text = if input_method == InputMethod::TextFile {
        let mut final_lines = Vec::new();
        for line in ocr_text.lines() {
            if line.contains("--- Result Summary ---") {
                break;
            }
            if line.contains(" = ") {
                continue;
            }
            final_lines.push(line);
        }
        final_lines.join("\n")
    } else {
        ocr_text.clone()
    };

    // If the Python script didn't already extract the math, we use our own detector here.
    if expressions.is_empty() {
        println!("\n--- Search for Math Problems ---");
        let mut detector = ExpressionDetector::new();
        let detected = detector.detect(&cleaned_ocr_text);

        for expr in &detected {
            expressions.push(expr.cleaned.clone());
        }
        detector.print_detected();
    } else {
        println!("Processing {} expressions identified by the OCR vision system.", expressions.len());
        for (i, expr) in expressions.iter().enumerate() {
            println!("  {}. {}", (b'a' + i as u8) as char, expr);
        }
    }

    if expressions.is_empty() {
        println!("No mathematical expressions were found. Nothing to solve.");
        return;
    }

    // Pass the extracted math to the internal math engine.
    println!("\n--- Solving Problems ---");
    let mut evaluator = ExpressionEvaluator::new();

    for expr in &expressions {
        evaluator.evaluate(expr);
    }

    // Display and save the results.
    println!("\n--- Finalizing Results ---");
    
    let output_mode = if let Some(path) = target_file {
        println!("Saving results and appending them to the source file: {}", path);
        OutputFormat::Both(path)
    } else {
        println!("Saving results to default log: {}", RESULTS_FILE);
        OutputFormat::Both(RESULTS_FILE.to_string())
    };

    let mut result_mgr = ResultManager::new(output_mode);
    result_mgr.set_source_name(&source_name);
    result_mgr.process_results(evaluator.get_results());
    result_mgr.display_results();

    println!("\nOperation Complete. {} problems successfully solved by the native engine.\n", evaluator.success_count());
}
