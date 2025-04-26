//! This is a library for reading input from standard input in Rust.
//! So far it only contains a function to read a line from standard input.
//! # Examples:
//! ```
//! use cli_utils::read_stdin;
//!
//! let input = read_stdin();
//! ```
//! # panics:
//! The function will panic if it fails to read a line from standard input.
//! So it will show an error message: "Failed to read line".

use std::io::{BufRead, BufReader};

/// This function reads a line from standard input and returns it as a String.
/// It will panic if reading fails to read the line.
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader: BufReader<std::io::StdinLock<'_>> = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}
