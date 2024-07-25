# Advanced Error Handling in Rust

Building on the basics of Rust's error handling, let's explore advanced techniques using the `thiserror` and `anyhow` crates to streamline custom error types and add contextual information.

## Advanced Error Handling with `thiserror`

### 1. Setting Up `thiserror`

Add `thiserror` to your `Cargo.toml`:

```toml
[dependencies]
thiserror = "1.0"
```

### 2. Defining Custom Error Types

Use `thiserror` to define custom error types concisely:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Custom error: {0}")]
    Custom(String),
}

fn parse_number(input: &str) -> Result<i32, MyError> {
    let number: i32 = input.parse()?;
    Ok(number)
}
```

**Explanation of Attributes:**

- `#[derive(Debug, Error)]`: Automatically implements the `Debug` and `Error` traits for the enum.
- `#[error("IO error: {0}")]`: Defines the error message format. `{0}` inserts the first argument, in this case, the contained error.
- `#[from]`: Automatically generates `From` trait implementations for easy conversion from the specified error type.

### 3. Adding Context with `anyhow`

Enhance error handling with context using `anyhow`:

```toml
[dependencies]
anyhow = "1.0"
```

```rust
use anyhow::{Context, Result};

fn read_and_parse_file(filename: &str) -> Result<i32> {
    let contents = std::fs::read_to_string(filename)
        .with_context(|| format!("Failed to read file: {}", filename))?;
    let number: i32 = contents.trim().parse()
        .with_context(|| format!("Failed to parse number from file: {}", filename))?;
    Ok(number)
}
```

## Combining `thiserror` and `anyhow`

Use both `thiserror` and `anyhow` for powerful error handling:

```rust
use std::fs::File;

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Custom error: {0}")]
    Custom(String),
}

fn read_file(filename: &str) -> Result<String, MyError> {
    let contents = std::fs::read_to_string(filename)
        .context("Failed to read file")?;
    Ok(contents)
}

fn parse_number(contents: &str) -> Result<i32, MyError> {
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn main() -> Result<(), MyError> {
    let filename = "test.txt";
    let contents = read_file(filename)?;
    let number = parse_number(&contents)?;
    println!("Parsed number: {}", number);
    Ok(())
}
```

## Summary

- **`thiserror`**: Simplifies defining custom error types.
- **`anyhow`**: Adds context to errors for easier debugging.
- **Combination**: Provides robust, concise, and maintainable error handling.

Use these advanced techniques to enhance your Rust applications, making them more resilient and easier to maintain.