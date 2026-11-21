//! rubberstamp
//!
//! Cargo test with [-- --nocapture] or with [-- --show-output]
//!
//! - cargo test --bin rubberstamp --release -- --nocapture
//! - cargo test --bin rubberstamp --release -- --show-output
//! - cargo run --bin rubberstamp --release
//! - cargo build --bin rubberstamp --release
//!

use rubberstamp::core::*;

use std::{result::Result, error::Error};

/// main
pub fn main() -> Result<(), Box<dyn Error>> {
  Ok(())
}

/// rubberstamp test
pub fn rubberstamp_test() -> Result<(), Box<dyn Error>> {
  assert_eq!(tempita_sub_test()?, ()); // same as lib
  assert_eq!(tempita_substitute_test()?, ()); // same as lib
  Ok(())
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use rubberstamp::core::*; // lib
  use crate::*; // main

  /// test rubberstamp
  #[test]
  fn test_rubberstamp() -> Result<(), Box<dyn Error>> {
    assert_eq!(py_test()?, ()); // same as lib
    assert_eq!(rubberstamp_test()?, ());
    Ok(())
  }
}
