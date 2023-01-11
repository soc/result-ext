[![crates.io](https://img.shields.io/crates/v/result-ext.svg)](https://crates.io/crates/result-ext)
[![API documentation](https://docs.rs/result-ext/badge.svg)](https://docs.rs/result-ext/)
![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-orange.svg)

# `result-ext`

## Introduction

This crate extends `Result` with additional methods, currently:

- `contains`
- `contains_err`
- `map_or2` (as a replacement for `map_or`)
- `map_or_else2` (as a replacement for `map_or_else`)

Its sister crate is [`option-ext`](https://github.com/soc/option-ext), which extends `Option`. 

## Requirements

Rust 1.0 or newer.

## Usage

#### Dependency

Add the library as a dependency to your project by inserting

```toml
result-ext = "0.2.0"
```

into the `[dependencies]` section of your Cargo.toml file.

#### Example

```rust
use result_ext::ResultExt;

fn example_contains() {
    use result_ext::ResultExt;

    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.contains(&2), true);

    let x: Result<u32, &str> = Ok(3);
    assert_eq!(x.contains(&2), false);

    let x: Result<u32, &str> = Err("Some error message");
    assert_eq!(x.contains(&2), false);
}

fn example_contains_err() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.contains_err(&"Some error message"), false);
    
    let x: Result<u32, &str> = Err("Some error message");
    assert_eq!(x.contains_err(&"Some error message"), true);
    
    let x: Result<u32, &str> = Err("Some other error message");
    assert_eq!(x.contains_err(&"Some error message"), false);
}

fn example_map_or2() {
    let x: Result<_, &str> = Ok("foo");
    assert_eq!(x.map_or2(|v| v.len(), 23), 3);
    
    let x: Result<&str, _> = Err("bar");
    assert_eq!(x.map_or2(|v| v.len(), 23), 23);
}

fn example_map_or_else2() {
    let k = 23;
    
    let x : Result<_, &str> = Ok("foo");
    assert_eq!(x.map_or_else2(|v| v.len(), |e| k * 2), 3);
    
    let x : Result<&str, _> = Err("bar");
    assert_eq!(x.map_or_else2(|v| v.len(), |e| k * 2), 46);  
}
```
