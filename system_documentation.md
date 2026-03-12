# Math Automation System - Technical Overview

This guide explains how the **Screen-Aware Math Automation System** is built and how the different parts work together. This system is designed for **Treeleaf AI** to automatically solve math equations using **OCR Tesseract** and the **Enigo Rust library**.

---

## 📂 Project Structure

| File | What it does |
| :--- | :--- |
| `main.rs` | The main controller. It asks the user for input and coordinates the whole process. |
| `automation.rs` | Handles standard computer interactions like typing text using the Enigo Rust Library. |
| `expression_detector.rs` | Uses text patterns (Regex) to search for math equations in plain text documents. |
| `expression_evaluator.rs` | The "Math Engine". A custom parser that accurately evaluates the math formulas. |
| `result_manager.rs` | Formats all final answers and saves them strictly to a results file. |
| `screen_capture.rs` | Takes screenshots and communicates with the Python OCR script. |

---

## 🧠 System Features & Flow

There are 3 main ways to perform the automation:
1. **By image path**: Passing a direct file path to an image for OCR reading.
2. **By taking screenshot**: Triggering a screenshot via the system's screen capture handler after an automated 5-second countdown.
3. **By using the .txt files**: Provide a direct path to a `.txt` file containing math equations.

### 1. Vision and Extraction (OCR)
When an image is provided, the system uses a **Python script** (`ocr_engine.py`) to process the file. 
- It uses **OpenCV** to denoise the image and standardize lighting.
- It then uses **Tesseract OCR** to turn the graphic math problems into regular text.
- We included rules to ignore list numbering like "1." or "a." so only math is returned.

### 2. Built-in Fallback Strategy
Because images can sometimes be messy or files may be empty, the system implements a strict fallback behavior:
If **no real math equations** are detected by either OCR or the text file parser, the code will use a **sample text block** (with pre-written math) as a failsafe to demonstrate the engine's capability without crashing.

### 3. Solving (The Engine)
Instead of typing into a separate calculator app, we built a **Recursive Descent Parser** in Rust to solve math perfectly. 
- It respects the correct order of operations (Multiplication and Division before Addition and Subtraction).
- It handles brackets `( )` and even supports negative signs.
- This creates true cross-platform behavior so the software works beautifully on **Windows, Mac, and Linux** using pure Rust code.

### 4. Output
Finally, the **Result Manager** gathers all calculated answers. It displays a summary on your console window and appends the answers securely to the main file at `output/results.txt`. If you chose a `.txt` file as input, the answers can be added directly back to your source file as an organized list.

---

## 🛠️ Main Requirements
- **Rust Compiler**: To build and run the main application logic and Enigo automations.
- **Python 3**: For handling the specific OpenCV image enhancements.
- **Tesseract OCR**: Pre-installed on the host operating system to transcribe image pixels into characters.
- **requirements.txt**: Manages the Python libraries securely so they can be installed with a single command.
- **test_images_files**: Any `.png` or `.txt` you provide for testing the system.

---
*Document prepared for Treeleaf AI technical review.*
