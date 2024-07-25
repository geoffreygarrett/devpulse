# Advanced Error Handling in Rust with `snafu`

Building on basic and advanced error handling with `thiserror` and `anyhow`, it's time to explore `snafu` for even more
powerful error management. Here’s how `snafu` stands out and how to use it effectively in Rust.

## Why `snafu`?

- **Structured and Context-Aware**: `snafu` makes it easy to generate errors with context, especially useful when the
  same underlying error can occur in different scenarios.
- **Custom Error Types**: Flexibly define custom error types.
- **Backtraces**: Automatically capture backtraces for better debugging.
- **Suitable for Libraries and Applications**: Works well in both small and large projects.
- **`no_std` Compatibility**: Usable in environments without the standard library.

## Getting Started with `snafu`

### Quick Setup

Add `snafu` to `Cargo.toml`:

```toml
[dependencies]
snafu = "0.6"
```

### Using `Whatever` for Quick Error Reporting

`Whatever` type and `whatever!` macro make error reporting simple:

```rust
use snafu::{prelude::*, Whatever};

fn is_valid_id(id: u16) -> Result<(), Whatever> {
    if id < 10 {
        whatever!("ID may not be less than 10, but it was {id}");
    }
    Ok(())
}
```

Wrap other errors with `Whatever`:

```rust
use snafu::{prelude::*, Whatever};

fn read_config_file(path: &str) -> Result<String, Whatever> {
    std::fs::read_to_string(path)
        .with_whatever_context(|_| format!("Could not read file {path}"))
}
```

`Whatever` tracks a backtrace for every error:

```rust
use snafu::{prelude::*, ErrorCompat, Whatever};

if let Err(e) = returns_an_error() {
eprintln ! ("An error occurred: {e}");
if let Some(bt) = ErrorCompat::backtrace( & e) {
eprintln ! ("{bt}");
}
}
```

### Creating Custom Error Types

When `Whatever` isn't enough, create custom error types by deriving `Snafu`.

#### Struct Style

Define custom error types using structs:

```rust
use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(display("ID may not be less than 10, but it was {id}"))]
struct InvalidIdError {
    id: u16,
}

fn is_valid_id(id: u16) -> Result<(), InvalidIdError> {
    ensure!(id >= 10, InvalidIdSnafu { id });
    Ok(())
}
```

**Explanation**:

- `#[snafu(display("..."))]`: Formats the error message.
- `InvalidIdSnafu`: Context selector type created by `Snafu`.

#### Wrapping Underlying Errors

Add a `source` field to wrap underlying errors:

```rust
use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(display("Could not read file {path}"))]
struct ConfigFileError {
    source: std::io::Error,
    path: String,
}

fn read_config_file(path: &str) -> Result<String, ConfigFileError> {
    std::fs::read_to_string(path).context(ConfigFileSnafu { path })
}
```

#### Enum Style

For handling multiple error types, use enums:

```rust
use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidId { id: u16 },
}

fn is_valid_id(id: u16) -> Result<(), Error> {
    ensure!(id >= 10, InvalidIdSnafu { id });
    Ok(())
}
```

### Combining Structs and Enums

Combine struct and enum error types for flexible error handling:

```rust
use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidId { id: u16 },

    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
        #[snafu(source(from(Box < dyn std::error::Error >, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },
}

fn is_valid_id(id: u16) -> Result<(), Error> {
    ensure!(id >= 10, InvalidIdSnafu { id });
    whatever!("Just kidding... this function always fails!");
    Ok(())
}
```

### Making Types `Send` and `Sync`

Ensure error types are `Send` and `Sync` for multithreaded programs:

```rust
#[snafu(source(from(Box < dyn std::error::Error + Send + Sync >, Some)))]
```

## Next Steps

For deeper understanding, read the documentation for the `Snafu` macro and
the [user’s guide](https://docs.rs/snafu/latest/snafu/guide/index.html).

## Summary

- **`snafu`**: Great for structured, context-aware error handling.
- **Custom error types**: Create detailed error types with minimal boilerplate.
- **Context selectors**: Add specific context to errors for better debugging.
- **Backtraces**: Automatically track backtraces for errors.
- **Flexible error handling**: Combine struct and enum error types for versatile error management.

Use `snafu` to enhance your Rust projects with robust, maintainable error handling. This completes the series on error
handling in Rust, bridging from basic techniques with `Result` and `Option`, to advanced handling with `thiserror`
and `anyhow`, and finally to highly structured error management with `snafu`.