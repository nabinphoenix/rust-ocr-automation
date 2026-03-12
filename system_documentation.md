# Math Automation System - Technical Overview

This guide explains how the **Screen-Aware Math Automation System** is built and how the different parts work together. This system is designed for **Treeleaf AI** to solve math problems automatically from images or text.

---

## 📂 Project Structure

| File | What it does |
| :--- | :--- |
| `main.rs` | The main controller. It asks the user for input and coordinates the whole process. |
| `automation.rs` | Handles computer interactions like typing and mouse movement. |
| `expression_detector.rs` | Uses patterns (Regex) to search for math equations in plain text. |
| `expression_evaluator.rs` | The "Math Engine". It is a custom parser that solves equations. |
| `result_manager.rs` | Formats the final answers and saves them to a results file. |
| `screen_capture.rs` | Takes screenshots and communicates with the Python OCR script. |

---

## 🧠 How the System Works

### 1. Vision (OCR)
When an image is provided, the system uses a **Python script** (`ocr_engine.py`) to "read" the picture. 
- It uses **OpenCV** to clean the image (removing blur and sharpening characters).
- It uses **Tesseract OCR** to turn pixels into characters.
- It filtered results to ignore things like "1." or "a." so it only sees the math.

### 2. Detection
If we are reading from a text file instead of an image, the **Expression Detector** scans every word. It uses **Regular Expressions** to find sequences of digits and math symbols (like `+`, `-`, `*`). It is smart enough to skip standard text and only grab the equations.

### 3. Solving (The Engine)
Instead of using a simple calculator, we built a **Recursive Descent Parser** in Rust. 
- It follows **BODMAS/PEMDAS** rules (Multiplication before Addition).
- It can handle brackets `( )` and even leading minus signs like `-5 + 10`.
- Because it is written in pure Rust, it works perfectly on **Windows, Mac, and Linux** without needing extra software.

### 4. Output
Finally, the **Result Manager** collects all the answers. It prints a clean summary to your screen and appends the same summary to `output/results.txt`. If you provided a text file as input, it can even append the answers directly to that file.

---

## 🛠️ Requirements
- **Rust**: For the main logic.
- **Python 3**: For image processing.
- **Tesseract OCR**: For reading text from images.
- **Requirements.txt**: All Python libraries are listed here for easy installation.

---
*Document prepared for Treeleaf AI technical review.*
