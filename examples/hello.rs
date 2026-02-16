// PlayJS Rust/WASM Example
// This file demonstrates Rust code that can be compiled and run in PlayJS

fn main() {
    println!("Hello from Rust/WASM!");

    // Example: Calculate Fibonacci sequence
    let n = 10;
    println!("Fibonacci sequence up to {}:", n);
    for i in 0..n {
        println!("{}: {}", i, fibonacci(i));
    }

    // Example: Vector operations
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("\nSum of {:?} = {}", numbers, sum);

    // Example: String manipulation
    let message = "PlayJS supports Rust!";
    println!("\n{}", message.to_uppercase());
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }
}
