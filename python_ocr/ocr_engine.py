# OCR Script - Extracts math equations from images.
# This script uses Tesseract OCR to find text and regex to filter out math.

import sys
import os
import re
import cv2
import numpy as np
from PIL import Image

try:
    import pytesseract
    import json
except ImportError:
    print("Error: Missing required Python libraries (pytesseract, json).", file=sys.stderr)
    sys.exit(1)

# Common places where Tesseract is installed on Windows/Linux.
TESSERACT_PATHS = [
    r"C:\Program Files\Tesseract-OCR\tesseract.exe",
    r"C:\Program Files (x86)\Tesseract-OCR\tesseract.exe",
    r"C:\Users\{}\AppData\Local\Tesseract-OCR\tesseract.exe".format(os.getenv("USERNAME", "")),
    "/usr/bin/tesseract",
]

def setup_tesseract():
    """Locate the tesseract executable on the system."""
    for path in TESSERACT_PATHS:
        if os.path.exists(path):
            pytesseract.pytesseract.tesseract_cmd = path
            return True
    try:
        pytesseract.get_tesseract_version()
        return True
    except:
        return False

def preprocess_image(img):
    """Clean the image to make it easier for the OCR to read."""
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

    # Upscale the image slightly for better character recognition.
    gray = cv2.resize(gray, None, fx=2.0, fy=2.0, interpolation=cv2.INTER_CUBIC)

    # Use adaptive thresholding to separate text from background noise.
    thresh = cv2.adaptiveThreshold(
        gray, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, 
        cv2.THRESH_BINARY, 31, 10
    )

    return thresh

def extract_text(image):
    """Run Tesseract on the processed image block."""
    pil_img = Image.fromarray(image)
    # Using PSM 6 to assume the image is a single block of text (good for math lists).
    custom_config = r'--psm 6'
    return pytesseract.image_to_string(pil_img, config=custom_config)

def extract_math_from_text(text):
    """Parse raw text and find valid math expressions line-by-line."""
    results = []
    seen = set()

    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue

        # Standardize different math symbols to basic ASCII (*, /, -).
        line = line.replace("×", "*").replace("x", "*").replace("X", "*")
        line = line.replace("÷", "/").replace("−", "-").replace("–", "-")
        
        # Remove serial numbers (like 1., a., b) ) from the start of the line.
        line = re.sub(r'^(?:[Nn]o\.?\s*)?[a-zA-Z\d]+[\.\)]\s*', '', line)
        
        # Regex to find sequences of digits and math operators.
        pattern = r"[\d\(\+\-\*\/][\d\+\-\*\/\(\)\.\s]*[\d\)\%]"
        matches = re.findall(pattern, line)
        
        for m in matches:
            # An expression must have at least one operator (+, -, *, /) to be valid.
            if any(op in m for op in "+-*/"):
                cleaned = re.sub(r'\s+', '', m)
                
                # Fix cases where multiplication was implied but symbols are missing.
                cleaned = re.sub(r'(\d)(\()', r'\1*\2', cleaned)
                cleaned = re.sub(r'(\))(\d)', r'\1*\2', cleaned)

                if len(cleaned) >= 3 and cleaned not in seen:
                    results.append(cleaned)
                    seen.add(cleaned)
    
    return results

def safe_eval(expr):
    """Solve the math string using Python's evaluator with safety restrictions."""
    try:
        if not re.match(r'^[\d\+\-\*\/\(\)\.]+$', expr):
            return "Error: Invalid Characters"
        
        # Only allow basic math evaluation.
        val = eval(expr, {"__builtins__": {}}, {})
        if isinstance(val, (int, float)):
            if isinstance(val, float) and val.is_integer():
                return int(val)
            return round(val, 4)
        return str(val)
    except:
        return "Error: Could not solve"

def main():
    if len(sys.argv) < 2:
        print("Usage: python ocr_engine.py <image_path>", file=sys.stderr)
        sys.exit(1)

    image_path = sys.argv[1]
    if not setup_tesseract():
        print("System Error: Tesseract OCR is not installed or not in PATH.", file=sys.stderr)
        sys.exit(1)

    img = cv2.imread(image_path)
    if img is None:
        print(f"Error: Could not load image file at {image_path}", file=sys.stderr)
        sys.exit(1)

    processed = preprocess_image(img)
    raw_text = extract_text(processed)
    expressions = extract_math_from_text(raw_text)

    # Solve all identified expressions.
    final_results = []
    for expr in expressions:
        val = safe_eval(expr)
        final_results.append({"expression": expr, "value": str(val)})

    # Output back to the Rust main process via JSON.
    output = {
        "raw_ocr": raw_text,
        "results": final_results
    }
    print(json.dumps(output))

if __name__ == "__main__":
    main()