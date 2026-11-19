#![doc(html_root_url = "https://docs.rs/rubberstamp/0.0.1")]
//! rubberstamp
//!
//! Cargo test with [-- --nocapture] or with [-- --show-output]
//!
//! cargo run --bin rubberstamp --release
//! cargo build --bin rubberstamp --release
//!

pub mod core;

#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use super::core::*;

  #[test]
  fn test_lib() -> Result<(), Box<dyn Error>> {
    assert_eq!(lib_test()?, ());
    Ok(())
  }
}
