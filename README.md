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
3.  **Tesseract OCR**: The "eye" that reads text.
    *   **Windows**: [Download installer](https://github.com/UB-Mannheim/tesseract/wiki).
    *   **Mac**: Run `brew install tesseract`.
    *   **Linux**: Run `sudo apt install tesseract-ocr`.
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

> [!TIP]
> **Pro Tip**: If the program misses a number, make sure your screenshot or image is clear and not too blurry. High-quality screenshots always work best!

---
*Created for efficient learning and automation for Treeleaf AI*
