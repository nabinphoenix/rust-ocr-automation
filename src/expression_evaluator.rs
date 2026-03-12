// Expression evaluator - parses and computes math expressions
// Supports +, -, *, / and parentheses

// Enum for tokens (the pieces of an expression)
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

// Enum for the result of evaluating an expression
#[derive(Debug, Clone)]
pub enum EvalResult {
    Success(f64),
    Error(String),
}

// Struct that evaluates expressions and stores results
pub struct ExpressionEvaluator {
    results: Vec<(String, EvalResult)>, // vector of tuples: (expression, result)
}

impl ExpressionEvaluator {
    pub fn new() -> Self {
        ExpressionEvaluator {
            results: Vec::new(),
        }
    }

    // Evaluate one expression string
    pub fn evaluate(&mut self, expression: &str) -> EvalResult {
        println!("  Evaluating: {}", expression);

        // First, break the expression into tokens
        let tokens = match tokenize(expression) {
            Ok(t) => t,
            Err(e) => {
                let result = EvalResult::Error(e);
                self.results.push((expression.to_string(), result.clone()));
                return result;
            }
        };

        // Then parse and compute the result
        let mut parser = Parser::new(tokens);
        let result = match parser.parse_expression() {
            Ok(value) => {
                println!("  {} = {}", expression, value);
                EvalResult::Success(value)
            }
            Err(e) => {
                println!("  Error: {}", e);
                EvalResult::Error(e)
            }
        };

        self.results.push((expression.to_string(), result.clone()));
        result
    }

    // Evaluate a list of expressions at once
    #[allow(dead_code)]
    pub fn evaluate_batch(&mut self, expressions: &[String]) -> Vec<(String, EvalResult)> {
        let mut batch: Vec<(String, EvalResult)> = Vec::new();

        for expr in expressions {
            let result = self.evaluate(expr);
            batch.push((expr.clone(), result));
        }

        batch
    }

    pub fn get_results(&self) -> &Vec<(String, EvalResult)> {
        &self.results
    }

    // New helper to inject results from external sources (like Calculator)
    pub fn evaluate_with_forced_result(&mut self, expression: &str, value: f64) {
        self.results.push((expression.to_string(), EvalResult::Success(value)));
    }

    // New helper to inject errors from external sources
    pub fn evaluate_with_error(&mut self, expression: &str, error: String) {
        self.results.push((expression.to_string(), EvalResult::Error(error)));
    }

    // Count how many evaluations succeeded
    pub fn success_count(&self) -> usize {
        let mut count: usize = 0;
        for (_, result) in &self.results {
            match result {
                EvalResult::Success(_) => count += 1,
                EvalResult::Error(_) => {}
            }
        }
        count
    }

    #[allow(dead_code)]
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}

// -------------------------------------------------------
// Tokenizer - turns a string like "2+3" into tokens
// -------------------------------------------------------

fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = expression.chars().collect();
    let length = chars.len();
    let mut i: usize = 0;

    while i < length {
        let ch = chars[i];

        match ch {
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                i += 1;
            }

            // Build a number from consecutive digits
            '0'..='9' | '.' => {
                let mut num_str = String::new();

                while i < length && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    num_str.push(chars[i]);
                    i += 1;
                }

                let number: f64 = num_str.parse()
                    .map_err(|_| format!("Bad number: {}", num_str))?;
                tokens.push(Token::Number(number));
            }

            '+' => { tokens.push(Token::Plus); i += 1; }
            '-' => {
                // Check if this minus is a negative sign (not subtraction)
                let is_negative = tokens.is_empty()
                    || matches!(tokens.last(), Some(Token::Plus) | Some(Token::Minus)
                        | Some(Token::Multiply) | Some(Token::Divide) | Some(Token::LeftParen));

                if is_negative && i + 1 < length && (chars[i + 1].is_ascii_digit() || chars[i + 1] == '.') {
                    let mut num_str = String::from("-");
                    i += 1;
                    while i < length && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        num_str.push(chars[i]);
                        i += 1;
                    }
                    let number: f64 = num_str.parse()
                        .map_err(|_| format!("Bad number: {}", num_str))?;
                    tokens.push(Token::Number(number));
                } else {
                    tokens.push(Token::Minus);
                    i += 1;
                }
            }
            '*' => { tokens.push(Token::Multiply); i += 1; }
            '/' => { tokens.push(Token::Divide); i += 1; }
            '(' => { tokens.push(Token::LeftParen); i += 1; }
            ')' => { tokens.push(Token::RightParen); i += 1; }

            _ => {
                return Err(format!("Unexpected character: '{}'", ch));
            }
        }
    }

    if tokens.is_empty() {
        return Err("Empty expression".to_string());
    }

    Ok(tokens)
}

// -------------------------------------------------------
// Parser - handles operator precedence
// Order: parentheses first, then * and /, then + and -
// -------------------------------------------------------

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    // Look at the current token without moving forward
    fn peek(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }

    // Take the current token and move to the next one
    fn consume(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    // Handle + and - (lowest priority)
    pub fn parse_expression(&mut self) -> Result<f64, String> {
        let mut left = self.parse_term()?;

        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = left + right;
                }
                Some(Token::Minus) => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = left - right;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Handle * and / (higher priority than + and -)
    // Also handles implicit multiplication like 3(4+5)
    fn parse_term(&mut self) -> Result<f64, String> {
        let mut left = self.parse_factor()?;

        loop {
            match self.peek() {
                Some(Token::Multiply) => {
                    self.consume();
                    let right = self.parse_factor()?;
                    left = left * right;
                }
                Some(Token::Divide) => {
                    self.consume();
                    let right = self.parse_factor()?;

                    if right == 0.0 {
                        return Err("Division by zero".to_string());
                    }

                    left = left / right;
                }
                Some(Token::LeftParen) => {
                    // Implicit multiplication: 3(4+5) -> 3 * (4+5)
                    let right = self.parse_factor()?;
                    left = left * right;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Handle numbers, parentheses, and unary minus (highest priority)
    fn parse_factor(&mut self) -> Result<f64, String> {
        match self.consume() {
            Some(Token::Number(n)) => Ok(n),
            Some(Token::Minus) => {
                // Unary minus: -factor
                let value = self.parse_factor()?;
                Ok(-value)
            }
            Some(Token::LeftParen) => {
                let value = self.parse_expression()?;
                match self.consume() {
                    Some(Token::RightParen) => Ok(value),
                    _ => Err("Missing closing parenthesis".to_string()),
                }
            }
            Some(token) => Err(format!("Unexpected token: {:?}", token)),
            None => Err("Unexpected end of expression".to_string()),
        }
    }
}

// -------------------------------------------------------
// Tests
// -------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let mut eval = ExpressionEvaluator::new();
        match eval.evaluate("2+3") {
            EvalResult::Success(v) => assert_eq!(v, 5.0),
            EvalResult::Error(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_subtraction() {
        let mut eval = ExpressionEvaluator::new();
        match eval.evaluate("1548-741") {
            EvalResult::Success(v) => assert_eq!(v, 807.0),
            EvalResult::Error(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_parentheses() {
        let mut eval = ExpressionEvaluator::new();
        match eval.evaluate("(500*2)/5") {
            EvalResult::Success(v) => assert_eq!(v, 200.0),
            EvalResult::Error(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_chained_operations() {
        let mut eval = ExpressionEvaluator::new();
        match eval.evaluate("123+456-78") {
            EvalResult::Success(v) => assert_eq!(v, 501.0),
            EvalResult::Error(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_division_by_zero() {
        let mut eval = ExpressionEvaluator::new();
        match eval.evaluate("10/0") {
            EvalResult::Error(_) => {} // this is expected
            EvalResult::Success(_) => panic!("Should have failed"),
        }
    }
}
