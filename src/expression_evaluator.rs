// This module parses and solves mathematical expressions.
// It handles basic arithmetic: addition (+), subtraction (-),
// multiplication (*), division (/), and uses parentheses for grouping.

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

#[derive(Debug, Clone)]
pub enum EvalResult {
    Success(f64),
    Error(String),
}

// Manages the state of evaluations and stores the answers.
pub struct ExpressionEvaluator {
    results: Vec<(String, EvalResult)>,
}

impl ExpressionEvaluator {
    pub fn new() -> Self {
        ExpressionEvaluator {
            results: Vec::new(),
        }
    }

    // Takes a string like "(2+3)*4" and returns the solved value.
    pub fn evaluate(&mut self, expression: &str) -> EvalResult {
        println!("  Evaluating: {}", expression);

        // Break the string into small tokens (numbers and operators).
        let tokens = match tokenize(expression) {
            Ok(t) => t,
            Err(e) => {
                let result = EvalResult::Error(e);
                self.results.push((expression.to_string(), result.clone()));
                return result;
            }
        };

        // Parse the tokens into a numerical result following math rules.
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

    pub fn get_results(&self) -> &Vec<(String, EvalResult)> {
        &self.results
    }

    // Records how many problems were solved correctly.
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
}

// --- Tokenizer Section ---
// Converts a raw string into a list of math symbols.

fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = expression.chars().collect();
    let length = chars.len();
    let mut i: usize = 0;

    while i < length {
        let ch = chars[i];

        match ch {
            // IGNORE spaces.
            ' ' | '\t' | '\n' | '\r' => {
                i += 1;
            }

            // Build a multi-digit number (e.g. "123.45").
            '0'..='9' | '.' => {
                let mut num_str = String::new();

                while i < length && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    num_str.push(chars[i]);
                    i += 1;
                }

                let number: f64 = num_str.parse()
                    .map_err(|_| format!("Invalid number format: {}", num_str))?;
                tokens.push(Token::Number(number));
            }

            '+' => { tokens.push(Token::Plus); i += 1; }
            '-' => {
                // Determine if this is a negative number or a subtraction sign.
                let is_negative_sign = tokens.is_empty()
                    || matches!(tokens.last(), Some(Token::Plus) | Some(Token::Minus)
                        | Some(Token::Multiply) | Some(Token::Divide) | Some(Token::LeftParen));

                if is_negative_sign && i + 1 < length && (chars[i + 1].is_ascii_digit() || chars[i + 1] == '.') {
                    let mut num_str = String::from("-");
                    i += 1;
                    while i < length && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        num_str.push(chars[i]);
                        i += 1;
                    }
                    let number: f64 = num_str.parse()
                        .map_err(|_| format!("Invalid negative number: {}", num_str))?;
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
                return Err(format!("Unknown symbol found: '{}'", ch));
            }
        }
    }

    if tokens.is_empty() {
        return Err("No math expression found".to_string());
    }

    Ok(tokens)
}

// --- Parser Section ---
// Handles operator precedence (BODMAS/PEMDAS).

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }

    fn consume(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    // Handles addition and subtraction.
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

    // Handles multiplication, division, and implicit multiplication like 2(5).
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
                        return Err("Logic Error: Cannnot divide by zero".to_string());
                    }

                    left = left / right;
                }
                Some(Token::LeftParen) => {
                    // Logic for 3(4) -> 3 * 4
                    let right = self.parse_factor()?;
                    left = left * right;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Handles the most basic units: numbers and bracketed content.
    fn parse_factor(&mut self) -> Result<f64, String> {
        match self.consume() {
            Some(Token::Number(n)) => Ok(n),
            Some(Token::Minus) => {
                // Supports leading minus sign like -(5+2)
                let value = self.parse_factor()?;
                Ok(-value)
            }
            Some(Token::LeftParen) => {
                let value = self.parse_expression()?;
                match self.consume() {
                    Some(Token::RightParen) => Ok(value),
                    _ => Err("Bracket error: Missing closing parenthesis".to_string()),
                }
            }
            Some(token) => Err(format!("Syntax error: Unexpected token {:?}", token)),
            None => Err("Parser error: Unexpected end of expression".to_string()),
        }
    }
}

// Tests to ensure the math engine is working as expected.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_math() {
        let mut eval = ExpressionEvaluator::new();
        match eval.evaluate("10+5*2") {
            EvalResult::Success(v) => assert_eq!(v, 20.0),
            _ => panic!("Test failed"),
        }
    }

    #[test]
    fn test_with_brackets() {
        let mut eval = ExpressionEvaluator::new();
        match eval.evaluate("(10+5)*2") {
            EvalResult::Success(v) => assert_eq!(v, 30.0),
            _ => panic!("Test failed"),
        }
    }
}
