//! {{crate}}
//!
//! Cargo test with [-- --nocapture] or with [-- --show-output]
//!
//! - cargo test --bin {{crate}} --release -- --nocapture
//! - cargo test --bin {{crate}} --release -- --show-output
//! - cargo run --bin {{crate}} --release
//! - cargo build --bin {{crate}} --release
//!

use {{crate}}::core::*;

use std::{result::Result, error::Error};

/// main
pub fn main() -> Result<(), Box<dyn Error>> {
  Ok(())
}

/// {{crate}} test
pub fn {{crate}}_test() -> Result<(), Box<dyn Error>> {
  Ok(())
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use {{crate}}::core::*; // lib
  use crate::*; // main

  /// test {{crate}}
  #[test]
  fn test_{{crate}}() -> Result<(), Box<dyn Error>> {
    assert_eq!({{crate}}_test()?, ());
    Ok(())
  }
}
