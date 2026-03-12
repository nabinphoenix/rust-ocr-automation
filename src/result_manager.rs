// This module organizes the results from the math engine.
// It formats the output and saves a summary to a text file.

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use crate::expression_evaluator::EvalResult;
use crate::expression_detector::index_to_label;

// Holds a single solved math problem.
#[derive(Debug, Clone)]
pub struct ResultEntry {
    pub label: char,
    pub expression: String,
    pub result: f64,
    pub is_integer: bool,
}

// Defines where the final answers should be displayed.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum OutputFormat {
    Console,
    File(String),
    Both(String),
}

pub struct ResultManager {
    entries: Vec<ResultEntry>,
    expression_map: HashMap<String, f64>,
    error_log: Vec<(String, String)>,
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

    // Set the name of the source (like 'image.png') to display in the header.
    pub fn set_source_name(&mut self, name: &str) {
        self.source_name = Some(name.to_string());
    }

    // Register a successfully solved equation.
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

    // Register an error for an equation that could not be solved.
    pub fn add_error(&mut self, expression: &str, error: &str) {
        self.error_log.push((expression.to_string(), error.to_string()));
    }

    // Takes a list of raw evaluation results and sorts them into successes and errors.
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

    // Builds the final text summary that the user sees.
    pub fn format_results(&self) -> String {
        let mut output = String::new();

        output.push_str("\n--- Result Summary ---\n");
        if let Some(ref source) = self.source_name {
            output.push_str(&format!("Output of {} :\n", source));
        }

        if self.entries.is_empty() {
            output.push_str("  (No mathematical results were found to display)\n");
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

        let total = self.entries.len() + self.error_log.len();
        output.push_str(&format!(
            "\nSummary: {} out of {} problems were solved successfully.\n",
            self.entries.len(), total
        ));

        // List any problems that could not be solved.
        if !self.error_log.is_empty() {
            output.push_str("\nIssues found:\n");
            for (expr, error) in &self.error_log {
                output.push_str(&format!("  Equation \"{}\" failed because: {}\n", expr, error));
            }
        }

        output
    }

    // Print to the console and/or save to a file.
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

    // Appends the summary to a text file without deleting previous contents.
    fn append_to_file(&self, path: &str, content: &str) {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path);

        match file {
            Ok(mut f) => {
                if let Err(e) = f.write_all(content.as_bytes()) {
                    println!("Warning: Could not save results to file {}: {}", path, e);
                }
            }
            Err(e) => println!("Warning: Could not open the results file {}: {}", path, e),
        }
    }
}
