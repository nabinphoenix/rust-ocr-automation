# Math Automation System Documentation

This document explains the architecture and functionality of the **Screen-Aware Math Automation System**, a Rust-based tool designed to detect and solve mathematical expressions from text files, images, or screenshots using a native, cross-platform math engine.

---

## 📂 File Overview

| File | Purpose |
| :--- | :--- |
| `main.rs` | The entry point that coordinates input, detection, solving, and output. |
| `automation.rs` | Handles keyboard/mouse simulation and clipboard interaction. |
| `expression_detector.rs` | Uses Regex to find mathematical expressions within raw text. |
| `expression_evaluator.rs` | Internal mathematical parser and result storage. |
| `result_manager.rs` | Formats results, tracks the source (filename/screenshot), and saves them. |
| `screen_capture.rs` | Captures screenshots and interfaces with the external OCR engine. |

---

## 1. `main.rs`
The "brain" of the application. It manages the program flow and integrates all other modules.

### Key Functions
- **`main()`**: 
  - Displays the menu.
  - Retrieves text based on the user's choice (Image, Screenshot, or File).
  - Cleans the input text (ignores old results).
  - Triggers the Calculator automation.
  - Saves the finalized list to the text file.
- **`show_menu()`**: Loop that displays options to the user and ensures valid input.
- **`read_input(prompt: &str)`**: Helper function to read a string from the console and trim unnecessary characters or quotes.
- **`get_text_file_path()`**: Validates existence and readability of a specified text file.

---

## 2. `automation.rs`
A utility module for interacting with the Windows Operating System.

### Struct: `Automator`
- **`new()`**: Initializes the `Enigo` keyboard driver and the system clipboard.
- **`type_text(text: &str)`**: Simulates typing a string character by character with a human-like delay.
- **`key_click(key: Key)`**: Simulates a single key press (e.g., `Enter`, `Escape`).
- **`key_combination(mod, key)`**: Simulates "hotkeys" like `Alt+2` (Scientific mode) or `Ctrl+C` (Copy).
- **`get_clipboard()`**: Reads the current text content of the Windows clipboard.
- **`click_at(x, y)`**: Moves the mouse to specific coordinates and clicks.

---

## 3. `expression_detector.rs`
Responsible for finding "math" inside messy strings of text.

### Struct: `ExpressionDetector`
- **`new()`**: Defines robust Regex patterns. These patterns are designed to find long math chains on a **single line** and specifically avoid capturing serial numbers (like "1.").
- **`detect(text: &str)`**: Scans the input text using the defined patterns. It ensures that it doesn't "double count" overlapping expressions.
- **`clean_expression(raw: &str)`**: Normalizes the text for the Calculator. It converts `x` or `×` into `*`, and complex minus signs into standard `-`.

---

## 4. `expression_evaluator.rs`
The core calculation engine that makes the system cross-platform. It contains a full tokenizer and recursive descent parser.

### Struct: `ExpressionEvaluator`
- **`evaluate(expression)`**: The primary method that tokenizes, parses, and solves math strings.
- **`Basic Support`**: Supports Addition, Subtraction, Multiplication, Division, and Parentheses.
- **`tokenize()` / `Parser`**: Internal logic that handles math precedence (Multiplication/Division before Addition/Subtraction).
- **`Cross-Platform`**: Replaces the need for Windows Calculator, allowing the system to run on Mac and Linux.

---

## 5. `result_manager.rs`
Handles the final display and file writing.

### Struct: `ResultManager`
- **`set_source_name(name)`**: Records the name of the input image or file for the final report.
- **`format_results()`**: Generates the final table/list, including the source filename.
- **`display_results()`**: Prints the summary to the console and appends it to your `.txt` file using `append_to_file`.
- **`index_to_label()`**: Converts numbers (0, 1, 2) into letters (a, b, c) for the summary list.

---

## 6. `screen_capture.rs`
Uses the `screenshots` crate and a Python script to "see" the screen.

### Struct: `ScreenCapture`
- **`capture_screen()`**: Takes a PNG image of your primary monitor.
- **`run_ocr(image_path)`**: Calls an external Python script (`ocr_engine.py`) to extract text from the image. It uses the `std::process::Command` to trigger the Python environment.
- **`capture_and_ocr()`**: A helper that combines both steps into one command.

---

## ⚙️ How it all works together
1. **Input**: `main.rs` gets text via `screen_capture.rs` or directly from your file.
2. **Extraction**: `expression_detector.rs` finds the math expressions.
3. **Calculation**: `main.rs` uses the native **ExpressionEvaluator** to solve the math. This bypasses the Windows Calculator to ensure compatibility with Mac and Linux.
4. **Storage**: `expression_evaluator.rs` stores the success and error states.
5. **Output**: `result_manager.rs` writes a clean summary back to your text file.
