// PlayJS Rust Example: Advanced Calculator
// Demonstrates structs, enums, pattern matching, and error handling

use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}

#[derive(Debug)]
enum CalculatorError {
    DivisionByZero,
    InvalidOperation,
    Overflow,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Cannot divide by zero"),
            CalculatorError::InvalidOperation => write!(f, "Invalid operation"),
            CalculatorError::Overflow => write!(f, "Result overflow"),
        }
    }
}

struct Calculator {
    memory: f64,
    history: Vec<String>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            memory: 0.0,
            history: Vec::new(),
        }
    }

    fn calculate(&mut self, a: f64, b: f64, op: Operation) -> Result<f64, CalculatorError> {
        let result = match op {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => {
                if b == 0.0 {
                    return Err(CalculatorError::DivisionByZero);
                }
                a / b
            }
            Operation::Power => a.powf(b),
            Operation::Modulo => {
                if b == 0.0 {
                    return Err(CalculatorError::DivisionByZero);
                }
                a % b
            }
        };

        if result.is_infinite() || result.is_nan() {
            return Err(CalculatorError::Overflow);
        }

        let operation_str = format!("{} {:?} {} = {}", a, op, b, result);
        self.history.push(operation_str);
        self.memory = result;

        Ok(result)
    }

    fn get_memory(&self) -> f64 {
        self.memory
    }

    fn clear_memory(&mut self) {
        self.memory = 0.0;
    }

    fn show_history(&self) {
        println!("\n=== Calculation History ===");
        if self.history.is_empty() {
            println!("No calculations yet.");
        } else {
            for (i, entry) in self.history.iter().enumerate() {
                println!("{}. {}", i + 1, entry);
            }
        }
        println!();
    }
}

fn main() {
    println!("=== Advanced Calculator Demo ===\n");

    let mut calc = Calculator::new();

    // Perform calculations
    match calc.calculate(10.0, 5.0, Operation::Add) {
        Ok(result) => println!("10 + 5 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calc.calculate(20.0, 4.0, Operation::Multiply) {
        Ok(result) => println!("20 * 4 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calc.calculate(100.0, 3.0, Operation::Divide) {
        Ok(result) => println!("100 / 3 = {:.4}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calc.calculate(2.0, 8.0, Operation::Power) {
        Ok(result) => println!("2 ^ 8 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match calc.calculate(17.0, 5.0, Operation::Modulo) {
        Ok(result) => println!("17 % 5 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Test division by zero
    println!("\nTesting error handling:");
    match calc.calculate(10.0, 0.0, Operation::Divide) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error caught: {}", e),
    }

    // Show calculator state
    println!("\nCurrent memory: {}", calc.get_memory());
    calc.show_history();

    // Clear and verify
    calc.clear_memory();
    println!("Memory after clear: {}", calc.get_memory());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let mut calc = Calculator::new();
        assert_eq!(calc.calculate(5.0, 3.0, Operation::Add).unwrap(), 8.0);
    }

    #[test]
    fn test_division_by_zero() {
        let mut calc = Calculator::new();
        assert!(matches!(
            calc.calculate(10.0, 0.0, Operation::Divide),
            Err(CalculatorError::DivisionByZero)
        ));
    }

    #[test]
    fn test_memory() {
        let mut calc = Calculator::new();
        calc.calculate(10.0, 5.0, Operation::Add).unwrap();
        assert_eq!(calc.get_memory(), 15.0);
        calc.clear_memory();
        assert_eq!(calc.get_memory(), 0.0);
    }
}
