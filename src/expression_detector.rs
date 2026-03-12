// This module finds math equations inside a block of plain text using regular expressions.

use regex::Regex;

// Represents a math problem found in the text.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DetectedExpression {
    pub raw_text: String,    // The exact text we found.
    pub cleaned: String,     // Cleaned version (no spaces, standardized symbols).
    pub position: usize,     // Where it sits in the list.
}

pub struct ExpressionDetector {
    patterns: Vec<String>,
    detected: Vec<DetectedExpression>,
}

impl ExpressionDetector {
    pub fn new() -> Self {
        // We look for patterns that look like equations on a single line.
        let pattern_array: [&str; 4] = [
            // Matches long chains of numbers and operators.
            r"(?m)(?:\d+(?:\.\d+)?|\([\d\.[ \t]\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\)]+)[ \t]*[\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\)][ \t]*[\d\.\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\) \t]+",
            // Simple multiplication/addition between two numbers.
            r"\d+(?:\.\d+)?[ \t]*[\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\)][ \t]*\d+(?:\.\d+)?",
            // Content inside parentheses.
            r"\([\d\.\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\) \t]+\)",
            // Basic two-digit math without spaces.
            r"\d+[\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX]\d+",
        ];

        let mut patterns: Vec<String> = Vec::new();
        for p in &pattern_array {
            patterns.push(p.to_string());
        }

        ExpressionDetector {
            patterns,
            detected: Vec::new(),
        }
    }

    // Scans a block of text and pulls out all the math.
    pub fn detect(&mut self, text: &str) -> Vec<DetectedExpression> {
        self.detected.clear();

        // Used to prevent picking up the same equation twice.
        let mut already_found: Vec<(usize, usize)> = Vec::new(); 
        let mut count: usize = 0;

        for pattern_str in &self.patterns {
            let regex = match Regex::new(pattern_str) {
                Ok(r) => r,
                Err(_) => continue,
            };

            for mat in regex.find_iter(text) {
                let start = mat.start();
                let end = mat.end();

                // Skip if this range overlaps with something we already found.
                let mut overlapping = false;
                for (s, e) in &already_found {
                    if (start >= *s && start < *e) || (end > *s && end <= *e) {
                        overlapping = true;
                        break;
                    }
                }

                if !overlapping {
                    let matched = mat.as_str().to_string();
                    let cleaned = clean_expression(&matched);

                    let expr = DetectedExpression {
                        raw_text: matched.clone(),
                        cleaned: cleaned.clone(),
                        position: count,
                    };

                    self.detected.push(expr);
                    already_found.push((start, end));
                    count += 1;
                }
            }
        }

        self.detected.clone()
    }

    pub fn print_detected(&self) {
        println!("\nThe system detected {} math expressions:", self.detected.len());
        for (i, expr) in self.detected.iter().enumerate() {
            let label = index_to_label(i);
            println!("  {}. {}", label, expr.cleaned);
        }
    }
}

// Cleans up the raw text for the math engine.
fn clean_expression(raw: &str) -> String {
    let mut cleaned = String::new();

    for ch in raw.chars() {
        match ch {
            '0'..='9' | '.' => cleaned.push(ch),
            '+' => cleaned.push('+'),
            '-' | '\u{2212}' => cleaned.push('-'),
            '*' | '/' | '(' | ')' => cleaned.push(ch),
            'x' | 'X' | '×' => cleaned.push('*'), // Convert 'x' to '*'
            '÷' => cleaned.push('/'),             // Convert division symbol
            _ => {}
        }
    }

    cleaned
}

// Converts a list index (0, 1, 2) to a letter (a, b, c) for cleaner display.
pub fn index_to_label(index: usize) -> char {
    let labels: [char; 26] = [
        'a','b','c','d','e','f','g','h','i','j',
        'k','l','m','n','o','p','q','r','s','t',
        'u','v','w','x','y','z',
    ];
    if index < labels.len() { labels[index] } else { '?' }
}
