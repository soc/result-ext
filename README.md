[![crates.io](https://img.shields.io/crates/v/result-ext.svg)](https://crates.io/crates/result-ext)
[![API documentation](https://docs.rs/result-ext/badge.svg)](https://docs.rs/result-ext/)
![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-orange.svg)

# `result-ext`

## Introduction

This crate extends `Result` with additional methods, currently:

- `contains`
- `contains_err`

Its sister crate is [`option-ext`](https://github.com/soc/option-ext), which extends `Option`. 

## Requirements

Rust 1.0 or newer.

## Usage

#### Dependency

Add the library as a dependency to your project by inserting

```toml
result-ext = "0.1.0"
```

into the `[dependencies]` section of your Cargo.toml file.

#### Example

```rust
use result_ext::ResultExt;

fn example() {
    use result_ext::ResultExt;
    
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.contains(&2), true);
    
    let x: Result<u32, &str> = Ok(3);
    assert_eq!(x.contains(&2), false);
    
    let x: Result<u32, &str> = Err("Some error message");
    assert_eq!(x.contains(&2), false);

    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.contains_err(&"Some error message"), false);
    
    let x: Result<u32, &str> = Err("Some error message");
    assert_eq!(x.contains_err(&"Some error message"), true);
    
    let x: Result<u32, &str> = Err("Some other error message");
    assert_eq!(x.contains_err(&"Some error message"), false);
}
```
