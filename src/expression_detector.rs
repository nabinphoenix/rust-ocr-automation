// Expression detector - finds math expressions in text using regex

use regex::Regex;

// Struct for a single detected expression
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DetectedExpression {
    pub raw_text: String,    // What the regex matched
    pub cleaned: String,     // Cleaned up version (no spaces)
    pub position: usize,     // Index in the list
}

// Struct that does the actual detection
pub struct ExpressionDetector {
    patterns: Vec<String>,
    detected: Vec<DetectedExpression>,
}

impl ExpressionDetector {
    pub fn new() -> Self {
        // IMPROVED: Strict single-line patterns to avoid catching S.No or crossing lines
        let pattern_array: [&str; 4] = [
            // Long chains: Must contain at least one operator, strictly horizontal space
            r"(?m)(?:\d+(?:\.\d+)?|\([\d\.[ \t]\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\)]+)[ \t]*[\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\)][ \t]*[\d\.\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\) \t]+",
            // Simple binary operation
            r"\d+(?:\.\d+)?[ \t]*[\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\)][ \t]*\d+(?:\.\d+)?",
            // Parentheses block on one line
            r"\([\d\.\+\-\*\/\u{00D7}\u{00F7}\u{2212}xX\(\) \t]+\)",
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

    // Scan text and find all math expressions
    pub fn detect(&mut self, text: &str) -> Vec<DetectedExpression> {
        self.detected.clear();

        // Track found ranges (start, end) to avoid double counting fragments
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

                // Check if this math expression is already partially inside another one
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
        println!("\nI found {} expressions in your input:", self.detected.len());
        for (i, expr) in self.detected.iter().enumerate() {
            let label = index_to_label(i);
            println!("  {}. {}", label, expr.cleaned);
        }
    }
}

// Clean up: remove spaces and change 'x' to '*'
fn clean_expression(raw: &str) -> String {
    let mut cleaned = String::new();

    for ch in raw.chars() {
        match ch {
            '0'..='9' | '.' => cleaned.push(ch),
            '+' => cleaned.push('+'),
            '-' | '\u{2212}' => cleaned.push('-'),
            '*' | '/' | '(' | ')' => cleaned.push(ch),
            'x' | 'X' | '×' => cleaned.push('*'), // Multiply
            '÷' => cleaned.push('/'),             // Divide
            _ => {}
        }
    }

    cleaned
}

pub fn index_to_label(index: usize) -> char {
    let labels: [char; 26] = [
        'a','b','c','d','e','f','g','h','i','j',
        'k','l','m','n','o','p','q','r','s','t',
        'u','v','w','x','y','z',
    ];
    if index < labels.len() { labels[index] } else { '?' }
}
