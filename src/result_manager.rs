// Result manager - stores results and prints/saves them

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use crate::expression_evaluator::EvalResult;
use crate::expression_detector::index_to_label;

// Struct for one result entry
#[derive(Debug, Clone)]
pub struct ResultEntry {
    pub label: char,
    pub expression: String,
    pub result: f64,
    pub is_integer: bool,
}

// Enum for where to send the output
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum OutputFormat {
    Console,
    File(String),
    Both(String),
}

// Struct that manages all the results
pub struct ResultManager {
    entries: Vec<ResultEntry>,
    expression_map: HashMap<String, f64>,
    error_log: Vec<(String, String)>,   // tuples of (expression, error message)
    output_format: OutputFormat,
    source_name: Option<String>,
}

impl ResultManager {
    pub fn new(format: OutputFormat) -> Self {
        ResultManager {
            entries: Vec::new(),
            expression_map: HashMap::new(),
            error_log: Vec::new(),
            output_format: format,
            source_name: None,
        }
    }

    // Set the source name (e.g., filename) for the output header
    pub fn set_source_name(&mut self, name: &str) {
        self.source_name = Some(name.to_string());
    }

    // Add a successful result
    pub fn add_result(&mut self, expression: &str, result: f64) {
        let index = self.entries.len();
        let label = index_to_label(index);
        let is_integer = result == result.floor() && result.is_finite();

        let entry = ResultEntry {
            label,
            expression: expression.to_string(),
            result,
            is_integer,
        };

        self.entries.push(entry);
        self.expression_map.insert(expression.to_string(), result);
    }

    // Add a failed result
    pub fn add_error(&mut self, expression: &str, error: &str) {
        self.error_log.push((expression.to_string(), error.to_string()));
    }

    // Go through evaluation results and sort them into success/error
    pub fn process_results(&mut self, results: &[(String, EvalResult)]) {
        for (expr, eval_result) in results {
            match eval_result {
                EvalResult::Success(value) => {
                    self.add_result(expr, *value);
                }
                EvalResult::Error(msg) => {
                    self.add_error(expr, msg);
                }
            }
        }
    }

    // Build a formatted string of all results
    pub fn format_results(&self) -> String {
        let mut output = String::new();

        output.push_str("\n--- Result Summary ---\n");
        if let Some(ref source) = self.source_name {
            output.push_str(&format!("Output of {} :\n", source));
        }

        if self.entries.is_empty() {
            output.push_str("  (No results found)\n");
            return output;
        }

        for entry in &self.entries {
            let value_str = if entry.is_integer {
                format!("{}", entry.result as i64)
            } else {
                format!("{:.2}", entry.result)
            };
            output.push_str(&format!("  {}. {} = {}\n", entry.label, entry.expression, value_str));
        }

        // Summary line
        let total = self.entries.len() + self.error_log.len();
        output.push_str(&format!(
            "\nTotal Solved: {}/{} successful.\n",
            self.entries.len(), total
        ));

        // Show errors
        if !self.error_log.is_empty() {
            output.push_str("\nErrors encountered:\n");
            for (expr, error) in &self.error_log {
                output.push_str(&format!("  \"{}\" - {}\n", expr, error));
            }
        }

        output
    }

    // Print or save results based on the output format
    pub fn display_results(&self) {
        let formatted = self.format_results();

        match &self.output_format {
            OutputFormat::Console => {
                println!("{}", formatted);
            }
            OutputFormat::File(path) => {
                self.append_to_file(path, &formatted);
            }
            OutputFormat::Both(path) => {
                println!("{}", formatted);
                self.append_to_file(path, &formatted);
            }
        }
    }

    // Append results to a file (doesn't overwrite)
    fn append_to_file(&self, path: &str, content: &str) {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path);

        match file {
            Ok(mut f) => {
                if let Err(e) = f.write_all(content.as_bytes()) {
                    println!("Failed to write to {}: {}", path, e);
                }
            }
            Err(e) => println!("Failed to open file {}: {}", path, e),
        }
    }

    #[allow(dead_code)]
    pub fn lookup(&self, expression: &str) -> Option<&f64> {
        self.expression_map.get(expression)
    }

    #[allow(dead_code)]
    pub fn get_entries(&self) -> &Vec<ResultEntry> {
        &self.entries
    }

    #[allow(dead_code)]
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.error_log.len()
    }
}
