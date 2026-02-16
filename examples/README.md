# PlayJS Rust/WASM Examples

This directory contains Rust examples that can be compiled and executed in the PlayJS browser IDE.

## Available Examples

### hello.rs
Basic Rust program demonstrating:
- println! macro
- Fibonacci sequence calculation
- Vector operations
- String manipulation
- Unit tests

### sorting.rs
Sorting algorithm implementations:
- Bubble Sort
- Quick Sort
- Merge Sort
- Performance comparison with Rust's built-in sort
- Generic implementations that work with any Ord type

### calculator.rs
Advanced calculator with:
- Structs and enums
- Pattern matching
- Error handling with custom error types
- Memory and history tracking
- Multiple operations (add, subtract, multiply, divide, power, modulo)

### data_structures.rs
Custom data structure implementations:
- Stack (LIFO)
- Queue (FIFO)
- Singly Linked List
- Binary Search Tree with in-order traversal
- Comprehensive test coverage

## Running Examples

In the PlayJS terminal, you can run these examples using:

```bash
rust examples/<filename>.rs
```

Or simply compile and execute the code from the Rust/WASM tab using:

```bash
rust
```

## Compiling to WebAssembly

The `src-rust/lib.rs` file contains WASM-compatible code that can be compiled to WebAssembly:

```bash
wasm-pack build --target web
```

This will generate JavaScript bindings that can be used directly in the browser.

## Features Demonstrated

- **Ownership and Borrowing**: Rust's memory safety guarantees
- **Pattern Matching**: Using match expressions and enums
- **Generics**: Type-safe generic data structures
- **Error Handling**: Result types and custom errors
- **Traits**: Implementing Display, Ord, and custom traits
- **Testing**: Unit tests with #[cfg(test)]
- **WASM Integration**: Browser-compatible Rust code

## Requirements

- Rust 2021 Edition
- Dependencies listed in Cargo.toml
- wasm-bindgen for WASM compilation
