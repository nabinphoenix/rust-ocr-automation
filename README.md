# 🧮 Screen-Aware Math Automation System

Welcome to the **Screen-Aware Math Automation System**! This is a smart tool that "sees" math problems on your screen or in images and solves them for you automatically. 

It uses **OCR Tesseract** and the **Enigo Rust library** to provide a smooth, cross-platform experience across Windows, Mac, and Linux.

---

## 🌟 What makes this special?

*   **Native Math Engine**: We don't just open a calculator; we built one inside the program. It is fast, accurate, and works on any computer.
*   **Smart Vision (OCR)**: Uses Tesseract OCR to read math even if there are logos, pictures, or notes around the numbers.
*   **List Cleaning**: It is smart enough to ignore serial numbers like `1.`, `a.`, or `No. 1` at the start of your problems.
*   **Result Tracking**: Every calculation is saved in `output/results.txt` with a label telling you exactly which file or screenshot it came from.
*   **Fallback Strategy**: If no math is found or the image cannot be read, the program falls back to a built-in sample text to demonstrate its solving capabilities safely.

---

## 📋 Prerequisites (What you need)

Before you start, make sure you have these tools installed on your computer:

1.  **Rust**: The main engine. [Install from here](https://www.rust-lang.org/tools/install).
2.  **Python 3**: Used for reading text from images. [Install from here](https://www.python.org/downloads/).
3.  **Tesseract OCR**: The "eye" that reads text from images. This is **required** — the program will not work without it.
    *   **Windows**: [Download installer](https://github.com/UB-Mannheim/tesseract/wiki). During installation, make sure to check **"Add to system PATH"** so the program can find it automatically.
    *   **Mac**: Run `brew install tesseract`.
    *   **Linux**: Run `sudo apt install tesseract-ocr`.
    
    > **Note for Windows users**: If Tesseract is not in PATH, the script will try to find it in the default install folder (`C:\Program Files\Tesseract-OCR\`). If you installed it somewhere else, you may need to update the path inside `python_ocr/ocr_engine.py`.

4.  **Python Libraries**: You can install all needed libraries at once using the `requirements.txt` file:
    ```bash
    pip install -r requirements.txt
    ```

---

## 🚀 How to use it

There are 3 ways to perform the automation:

1. **By image path**: Type the path to an image file (e.g., `test_images_files/calculation.txt`).
2. **By taking screenshot**: Screen capture mode gives you 5 seconds to switch to your math problems before snapping a picture.
3. **By using the .txt files**: Give it a direct text file with math written inside.

To start the program, open your terminal in the project folder and run:
```bash
cargo run
```

Then, follow the menu to select your preferred option. The answers will appear directly on your screen, and a permanent copy will be saved in `output/results.txt`.

---

## 🛠️ Supported Operations

This version is optimized for **maximum accuracy** in basic math:
- ✅ Addition (`+`)
- ✅ Subtraction (`-`)
- ✅ Multiplication (`*` or `x`)
- ✅ Division (`/`)
- ✅ Parentheses `( )`
- ✅ Unary Minus (e.g., `-5 + 10`)

---

## 📁 Project Structure

*   `src/`: The core Rust code including the math engine and the Enigo automation setup.
*   `python_ocr/`: The Python script that runs OpenCV and Tesseract.
*   `test_images_files/`: A folder for you to put your sample math images or text files.
*   `output/`: Where your solved answers are saved.

---

## 💾 How Results are Saved

Every time you run the program, the answers are saved in **two places**:

1.  **On your screen** — the result summary is printed directly in the terminal when the program finishes.
2.  **In `output/results.txt`** — the same summary is also **appended** to this file. This means every run adds new results at the bottom without deleting the old ones.

If you use **Option 3 (text file input)**, the results are appended to **both** `output/results.txt` and the original text file you provided, so you can see the answers right next to the questions.

> **Note**: The `output/` folder is created automatically the first time you run the program. You do not need to create it yourself.

---

## 🦀 Rust Concepts Used

This project was built to make good use of core Rust language features:

*   **Structs**: Used to organize related data together. For example, `ExpressionEvaluator`, `ResultManager`, `ScreenCapture`, and `Automator` are all structs that group their data and behaviour in one place.
*   **Enums**: Used to represent choices clearly. `InputMethod` (Image, Screenshot, TextFile, Exit), `EvalResult` (Success or Error), and `OutputFormat` are all enums — making the code easy to read and safe to handle.
*   **Ownership**: Rust's ownership system is used throughout to ensure memory is managed correctly without a garbage collector. Each piece of data has exactly one owner at a time.
*   **Borrowing**: Functions take references (`&str`, `&mut self`) instead of copying data unnecessarily. For example, `evaluate(&mut self, expression: &str)` borrows the expression string rather than taking ownership of it.
*   **Pattern Matching**: `match` statements are used to handle different cases cleanly, like checking if an OCR result is a `Success` or an `Error` without using messy if-else chains.

---

> [!TIP]
> **Pro Tip**: If the program misses a number, make sure your screenshot or image is clear and not too blurry. High-quality screenshots always work best!

---
*Created for Treeleaf AI.*

