# OCR + Math Calculation Engine (REFINED V3)
# Focused on vertical line separation and clean math detection
# Usage: python ocr_engine.py <image_path>

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
    print("ERROR: pytesseract or json not found.", file=sys.stderr)
    sys.exit(1)

# -----------------------------
# Tesseract Detection
# -----------------------------
TESSERACT_PATHS = [
    r"C:\Program Files\Tesseract-OCR\tesseract.exe",
    r"C:\Program Files (x86)\Tesseract-OCR\tesseract.exe",
    r"C:\Users\{}\AppData\Local\Tesseract-OCR\tesseract.exe".format(os.getenv("USERNAME", "")),
    "/usr/bin/tesseract",
]

def setup_tesseract():
    for path in TESSERACT_PATHS:
        if os.path.exists(path):
            pytesseract.pytesseract.tesseract_cmd = path
            return True
    try:
        pytesseract.get_tesseract_version()
        return True
    except:
        return False

# -----------------------------
# Image Preprocessing (Balanced)
# -----------------------------
def preprocess_image(img):
    # Convert to grayscale
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

    # 1. Medium upscale (2x is often more stable than 3x)
    gray = cv2.resize(gray, None, fx=2.0, fy=2.0, interpolation=cv2.INTER_CUBIC)

    # 2. Simple Adaptive Thresholding (more robust for varied backgrounds)
    thresh = cv2.adaptiveThreshold(
        gray, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, 
        cv2.THRESH_BINARY, 31, 10
    )

    return thresh

# -----------------------------
# OCR Extraction
# -----------------------------
def extract_text(image):
    """Standard image_to_string is often better for simple vertical lists."""
    pil_img = Image.fromarray(image)
    # PSM 6: Assume a single uniform block of text (good for lists)
    custom_config = r'--psm 6'
    return pytesseract.image_to_string(pil_img, config=custom_config)

# -----------------------------
# Clean and Extract Math
# -----------------------------
def extract_math_from_text(text):
    results = []
    seen = set()

    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue

        # 1. Standardize symbols first
        line = line.replace("×", "*").replace("x", "*").replace("X", "*")
        line = line.replace("÷", "/").replace("−", "-").replace("–", "-")
        
        # 2. Remove Serial Numbers (1., a., b), No. 1)
        # We use a very aggressive regex for this to ensure the start of the line is clean
        line = re.sub(r'^(?:[Nn]o\.?\s*)?[a-zA-Z\d]+[\.\)]\s*', '', line)
        
        # 3. Match potential math expressions
        # Rule: Start with digit/paren, use only math symbols, end with digit/paren
        # [ \t]* ensures we don't jump across newlines
        pattern = r"[\d\(\+\-\*\/][\d\+\-\*\/\(\)\.\s]*[\d\)\%]"
        matches = re.findall(pattern, line)
        
        for m in matches:
            # Must have at least one operator
            if any(op in m for op in "+-*/"):
                cleaned = re.sub(r'\s+', '', m)
                
                # Heuristic: Fix missing multiplication
                cleaned = re.sub(r'(\d)(\()', r'\1*\2', cleaned)
                cleaned = re.sub(r'(\))(\d)', r'\1*\2', cleaned)

                if len(cleaned) >= 3 and cleaned not in seen:
                    results.append(cleaned)
                    seen.add(cleaned)
    
    return results

def safe_eval(expr):
    try:
        # Protect against non-math chars once more
        if not re.match(r'^[\d\+\-\*\/\(\)\.]+$', expr):
            return "Error: Invalid"
        
        val = eval(expr, {"__builtins__": {}}, {})
        if isinstance(val, (int, float)):
            if isinstance(val, float) and val.is_integer():
                return int(val)
            return round(val, 4)
        return str(val)
    except:
        return "Error"

# -----------------------------
# Main Entry
# -----------------------------
def main():
    if len(sys.argv) < 2:
        print("Usage: python ocr_engine.py <image_path>", file=sys.stderr)
        sys.exit(1)

    image_path = sys.argv[1]
    if not setup_tesseract():
        print("Tesseract OCR not found.", file=sys.stderr)
        sys.exit(1)

    img = cv2.imread(image_path)
    if img is None:
        sys.exit(1)

    processed = preprocess_image(img)
    raw_text = extract_text(processed)
    expressions = extract_math_from_text(raw_text)

    # Solve
    final_results = []
    for expr in expressions:
        val = safe_eval(expr)
        final_results.append({"expression": expr, "value": str(val)})

    output = {
        "raw_ocr": raw_text,
        "results": final_results
    }
    print(json.dumps(output))

if __name__ == "__main__":
    main()