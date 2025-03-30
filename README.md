# up_finder

[![Crates.io](https://img.shields.io/crates/v/up_finder)](https://crates.io/crates/up_finder)
[![Documentation](https://docs.rs/up_finder/badge.svg)](https://docs.rs/up_finder)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight Rust library for finding files or directories upward in the directory tree.

[English](README.md) | [中文](README.zh.md)

## Features

- Find files or directories in the current working directory and all parent directories
- Support for finding a single file or multiple files
- High-performance HashMap implementation using `rustc-hash`
- Concise builder API implemented with `typed-builder`
- No external system dependencies, pure Rust implementation

## Installation

Add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
up_finder = "0.0.1"
```

## Usage Examples

### Find a Single File

```rust
use up_finder::{UpFinder, FindUpKind};

// Create a UpFinder instance, default is to find files
let find_up = UpFinder::builder()
    .cwd(".")  // Start from the current directory
    .kind(FindUpKind::File)  // Optional, finding files is the default
    .build();

// Find package.json files upward
let paths = find_up.find_up("package.json");

// Print all found paths
println!("{:#?}", paths);
```

### Find Multiple Files

```rust
use up_finder::{UpFinder, FindUpKind};

// Create a UpFinder instance
let find_up = UpFinder::builder()
    .cwd("./src")  // Start from the src directory
    .build();

// Find multiple files simultaneously
let paths = find_up.find_up_multi(&["package.json", ".gitignore", "Cargo.toml"]);

// Result is a HashMap with file names as keys and lists of found paths as values
for (file_name, file_paths) in paths {
    println!("Found {} {} files:", file_paths.len(), file_name);
    for path in file_paths {
        println!("  - {}", path.display());
    }
}
```

### Find Directories

```rust
use up_finder::{UpFinder, FindUpKind};

// Create a UpFinder instance for finding directories
let find_up = UpFinder::builder()
    .cwd(".")
    .kind(FindUpKind::Dir)  // Set to find directories
    .build();

// Find "node_modules" directories upward
let paths = find_up.find_up("node_modules");

println!("{:#?}", paths);
```

## API Documentation

For detailed API documentation, visit [docs.rs/up_finder](https://docs.rs/up_finder).

## Contribution

Issues and pull requests are welcome!

## License

This project is licensed under the [MIT License](LICENSE).
