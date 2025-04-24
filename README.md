# binstring

A Rust library for storing binary data as a string.

[![Crates.io](https://img.shields.io/crates/v/binstring)](https://crates.io/crates/binstring)
[![Documentation](https://docs.rs/binstring/badge.svg)](https://docs.rs/binstring)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description

`binstring` provides a `BinString` type that wraps a `String` and provides conversion methods between binary data and strings. It's particularly useful when you need to store binary data in a string format while maintaining the ability to access it as both a string and bytes.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
binstring = "0.1"
```

## Usage

### Basic Usage

```rust
use binstring::BinString;

// Create from a string
let bin_str = BinString::new("hello");
assert_eq!(bin_str.as_str(), "hello");

// Create from bytes
let bytes = vec![104, 101, 108, 108, 111]; // "hello" in bytes
let bin_str = BinString::from_bytes(bytes);
assert_eq!(bin_str.as_bytes(), &[104, 101, 108, 108, 111]);
```

### Converting Between Types

```rust
use binstring::BinString;

// From String
let s = String::from("hello");
let bin_str = BinString::from(s);

// From &str
let bin_str = BinString::from("hello");

// From Vec<u8>
let bytes = vec![104, 101, 108, 108, 111];
let bin_str = BinString::from(bytes);

// From &[u8]
let bytes = &[104, 101, 108, 108, 111];
let bin_str = BinString::from(bytes);
```

### Accessing Data

```rust
use binstring::BinString;

let bin_str = BinString::new("hello");

// As string slice
assert_eq!(bin_str.as_str(), "hello");

// As byte slice
assert_eq!(bin_str.as_bytes(), &[104, 101, 108, 108, 111]);

// Get length
assert_eq!(bin_str.len(), 5);

// Check if empty
assert!(!bin_str.is_empty());
```

### Converting Back

```rust
use binstring::BinString;

let bin_str = BinString::new("hello");

// Get the underlying String
let s = bin_str.unwrap();
assert_eq!(s, "hello");

// Or use From trait
let s: String = bin_str.into();
assert_eq!(s, "hello");
```

## Safety

While storing invalid UTF-8 in a `String` is perfectly safe, attempting to treat the string as valid UTF-8 (for example, by displaying it or using string operations that assume valid UTF-8) may lead to undefined behavior.
