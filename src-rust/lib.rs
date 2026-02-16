//! PlayJS Rust/WASM Library
//!
//! This library provides core Rust functionality for the PlayJS browser IDE.
//! It demonstrates various Rust programming concepts and patterns.

pub mod algorithms;
pub mod math;
pub mod utils;

/// Version information for the PlayJS Rust library
pub const VERSION: &str = "0.1.0";

/// Initialize the WASM module
#[cfg(target_arch = "wasm32")]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Greet function for WASM binding
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from Rust/WASM, {}! 🦀", name)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn fibonacci_wasm(n: u32) -> u32 {
    algorithms::fibonacci(n)
}

/// Module containing algorithm implementations
pub mod algorithms {
    /// Calculate the nth Fibonacci number
    pub fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    /// Check if a number is prime
    pub fn is_prime(n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    /// Calculate factorial
    pub fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }
}

/// Module containing mathematical utilities
pub mod math {
    /// Calculate the greatest common divisor
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// Calculate the least common multiple
    pub fn lcm(a: u64, b: u64) -> u64 {
        (a * b) / gcd(a, b)
    }

    /// Calculate power using fast exponentiation
    pub fn power(base: i64, exp: u32) -> i64 {
        let mut result = 1;
        let mut base = base;
        let mut exp = exp;

        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            base *= base;
            exp /= 2;
        }
        result
    }
}

/// Module containing utility functions
pub mod utils {
    /// Check if a string is a palindrome
    pub fn is_palindrome(s: &str) -> bool {
        let s = s.to_lowercase();
        let chars: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).collect();
        chars == chars.iter().rev().cloned().collect::<Vec<char>>()
    }

    /// Count word occurrences in text
    pub fn word_count(text: &str) -> usize {
        text.split_whitespace().count()
    }

    /// Reverse a string
    pub fn reverse_string(s: &str) -> String {
        s.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(algorithms::fibonacci(0), 0);
        assert_eq!(algorithms::fibonacci(1), 1);
        assert_eq!(algorithms::fibonacci(10), 55);
    }

    #[test]
    fn test_is_prime() {
        assert!(algorithms::is_prime(7));
        assert!(!algorithms::is_prime(4));
        assert!(algorithms::is_prime(13));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(algorithms::factorial(5), 120);
        assert_eq!(algorithms::factorial(0), 1);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(math::gcd(48, 18), 6);
        assert_eq!(math::gcd(100, 50), 50);
    }

    #[test]
    fn test_palindrome() {
        assert!(utils::is_palindrome("racecar"));
        assert!(utils::is_palindrome("A man a plan a canal Panama"));
        assert!(!utils::is_palindrome("hello"));
    }
}
