// Main file - runs the whole program step by step

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

use std::collections::HashMap;
use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

const OUTPUT_DIR: &str = "output";
const RESULTS_FILE: &str = "output/results.txt";

// Enum to represent which input option the user picked
#[derive(Debug, Clone, PartialEq)]
enum InputMethod {
    ImagePath,
    Screenshot,
    TextFile,
    Exit,
}

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

// Reads a line from the user and returns it as a String
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Trim spaces and remove quotes
    input.trim().trim_matches('"').trim().to_string()
}

// Shows the menu and returns what the user picked
fn show_menu() -> InputMethod {
    println!("\n--- Select Input Method ---");
    println!("  1. Enter Image Path  (run OCR on an image)");
    println!("  2. Take Screenshot   (capture the screen)");
    println!("  3. Enter File Path   (read text from a file)");
    println!("  0. Exit              (close program)");

    loop {
        let choice = read_input("\nEnter your choice (1/2/3/0): ");

        match choice.as_str() {
            "1" => return InputMethod::ImagePath,
            "2" => return InputMethod::Screenshot,
            "3" => return InputMethod::TextFile,
            "0" | "exit" | "quit" => return InputMethod::Exit,
            _ => println!("Invalid choice. Please enter 1, 2, 3, or 0."),
        }
    }
}

// Asks the user for an image path and checks if it exists
fn get_image_path() -> String {
    loop {
        let path = read_input("Enter the image path: ");

        if Path::new(&path).exists() {
            let lower = path.to_lowercase();
            if lower.ends_with(".png") || lower.ends_with(".jpg")
                || lower.ends_with(".jpeg") || lower.ends_with(".bmp")
                || lower.ends_with(".tiff")
            {
                return path;
            } else {
                println!("This file might not be an image.");
                let confirm = read_input("Use it anyway? (y/n): ");
                if confirm.to_lowercase() == "y" {
                    return path;
                }
            }
        } else {
            println!("File not found: {}", path);
        }
    }
}

// Asks the user for a text file path and checks if it exists
fn get_text_file_path() -> String {
    loop {
        let path = read_input("Enter the text file path: ");

        if Path::new(&path).exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    if content.is_empty() {
                        println!("File is empty, pick another one.");
                    } else {
                        println!("File loaded: {} ({} characters)", path, content.len());
                        return path;
                    }
                }
                Err(e) => {
                    println!("Could not read file: {}", e);
                }
            }
        } else {
            println!("File not found: {}", path);
        }
    }
}

// Sample text used when OCR is not available
fn get_sample_text() -> String {
    String::from(
        "Invoice #12345\n\
         Item 1: 1548-741\n\
         Item 2: (500*2)/5\n\
         Adjustment: 123+456-78\n"
    )
}

fn main() {
    println!("\n=== Screen-Aware Math Automation System ===\n");

    // Choose Input
    let input_method = show_menu();

    // Handle Exit
    if input_method == InputMethod::Exit {
        println!("Goodbye!\n");
        return;
    }

    let mut target_file: Option<String> = None;

    // Process Input
    println!("\n--- Processing Input ---");

    // Step 2: Get text (OCR or File)
    let mut expressions: Vec<String> = Vec::new();
    let mut source_name = String::from("Unknown");
    let ocr_text: String = match input_method {

        InputMethod::ImagePath | InputMethod::Screenshot => {
            let path = if input_method == InputMethod::ImagePath {
                let p = get_image_path();
                source_name = Path::new(&p).file_name().and_then(|s| s.to_str()).unwrap_or(&p).to_string();
                p
            } else {
                println!("\nPreparing screenshot...");
                for i in (1..=5).rev() {
                    print!("Taking screenshot in {}... ", i);
                    io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
                println!("GO!\n");
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
                        // Try to parse JSON from Python
                        match serde_json::from_str::<OcrResponse>(&json_str) {
                            Ok(resp) => {
                                println!("OCR successful (JSON parsed)!");
                                // Extract expressions from the JSON results if possible
                                let mut seen = std::collections::HashSet::new();
                                for entry in &resp.results {
                                    // Clean slightly but keep original for display if helpful
                                    let clean = entry.expression.replace(" ", "");
                                    if !entry.expression.contains("Error") && !seen.contains(&clean) {
                                        // Use a more math-standard version of the expression
                                        let math_expr = entry.expression.clone();
                                        expressions.push(math_expr);
                                        seen.insert(clean);
                                    }
                                }
                                println!("  OCR engine identified {} potential expressions.", expressions.len());
                                resp.raw_ocr
                            }
                            Err(_) => {
                                println!("OCR successful (Raw text)!");
                                json_str
                            }
                        }
                    }
                    Err(e) => {
                        println!("OCR failed: {}. Using sample.", e);
                        get_sample_text()
                    }
                }
            }
        }

        InputMethod::TextFile => {
            let file_path = get_text_file_path();
            source_name = Path::new(&file_path).file_name().and_then(|s| s.to_str()).unwrap_or(&file_path).to_string();
            target_file = Some(file_path.clone());
            println!("Reading from: {}", file_path);

            match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(e) => {
                    println!("Failed to read file: {}. Using sample.", e);
                    get_sample_text()
                }
            }
        }
        
        _ => get_sample_text(),
    };

    // Step 2.5: Clean input text (ignore previous results)
    let cleaned_ocr_text = if input_method == InputMethod::TextFile {
        let mut final_lines = Vec::new();
        for line in ocr_text.lines() {
            // Stop if we hit the result summary from a previous run
            if line.contains("--- Result Summary ---") {
                break;
            }
            // Skip lines that already look like they have a result
            if line.contains(" = ") {
                continue;
            }
            final_lines.push(line);
        }
        final_lines.join("\n")
    } else {
        ocr_text.clone()
    };

    // Step 3: Find math expressions in the text (Only if we haven't found them via JSON OCR)
    println!("\n--- Finding Math Expressions ---");

    if expressions.is_empty() {
        let mut detector = ExpressionDetector::new();
        let detected = detector.detect(&cleaned_ocr_text);

        for expr in &detected {
            expressions.push(expr.cleaned.clone());
        }
        detector.print_detected();
    } else {
        println!("Using {} expressions found by OCR engine.", expressions.len());
        for (i, expr) in expressions.iter().enumerate() {
            println!("  {}. {}", (b'a' + i as u8) as char, expr);
        }
    }

    if expressions.is_empty() {
        println!("No expressions found in text.");
        return;
    }

    // Step 4: Evaluate each expression
    println!("\n--- Solving ---");

    // Step 4: Evaluate each expression using Native Cross-Platform Evaluator
    println!("\n--- Solving via Native Math Engine (Mac/Linux/Windows Compatible) ---");

    let mut evaluator = ExpressionEvaluator::new();

    for expr in &expressions {
        evaluator.evaluate(expr);
    }

    // Step 5: Show Results and Save
    println!("\n--- Results ---");
    
    // Determine output: either global file OR append to the input file
    let output_mode = if let Some(path) = target_file {
        println!("Saving results and appending to: {}", path);
        OutputFormat::Both(path)
    } else {
        println!("Saving results to: {}", RESULTS_FILE);
        OutputFormat::Both(RESULTS_FILE.to_string())
    };

    let mut result_mgr = ResultManager::new(output_mode);
    result_mgr.set_source_name(&source_name);
    result_mgr.process_results(evaluator.get_results());
    result_mgr.display_results();

    println!("\nAll done! {} expressions solved via Native Math Engine.\n", evaluator.success_count());
}
