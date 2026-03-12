# 🧮 Screen-Aware Math Automation System

Welcome to the **Screen-Aware Math Automation System**! This is a smart tool that "sees" math problems on your screen or in images and solves them for you automatically. 

It is built using **Rust** for speed and **Python** for its powerful "eyes" (OCR). Unlike typical tools, this one works everywhere—Windows, Mac, and Linux!

---

## 🌟 What makes this special?

*   **Native Math Engine**: We don't just open a calculator; we built one inside the program. It is fast, accurate, and works on any computer.
*   **Smart Vision (OCR)**: It can read math even if there are logos, pictures, or notes around the numbers.
*   **List Cleaning**: It is smart enough to ignore serial numbers like `1.`, `a.`, or `No. 1` at the start of your problems.
*   **Result Tracking**: Every calculation is saved in `output/results.txt` with a label telling you exactly which file or screenshot it came from.
*   **Cross-Platform**: Fully compatible with Windows, macOS, and Linux.

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

1.  **Open your terminal** in the project folder.
2.  **Run the program**:
    ```bash
    cargo run
    ```
3.  **Choose your input**:
    *   **Option 1**: Type the path to an image file (e.g., `test_images/homework.png`).
    *   **Option 2**: Take a screenshot. The program will wait 5 seconds so you can switch to the window you want to capture!
    *   **Option 3**: Give it a text file with math written inside.
4.  **Check your results**:
    *   The answers will appear on your screen.
    *   A permanent copy is saved in `output/results.txt`.

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

*   `src/`: The core Rust code (Math Engine & Automation).
*   `python_ocr/`: The Python "Vision" script.
*   `test_images/`: A folder for you to put your math images.
*   `output/`: Where your solved answers are saved.

> [!TIP]
> **Pro Tip**: If the program misses a number, make sure the image is clear and not too blurry. High-quality screenshots work best!

---
*Created for efficient learning and automation for Treeleaf AI*
