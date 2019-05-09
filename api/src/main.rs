extern crate api;

use api::{config::Config, error::Error, run};

/// Entry point for binary only.
/// Use `lib.rs` for testing.
fn main() -> Result<(), Error> {
  let config = Config::from_args()?;
  run(config)
}
