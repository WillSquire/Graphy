use crate::error::Error;
use argon2::{hash_encoded, verify_encoded, Config, ThreadMode, Variant, Version};

pub type Hasher = Box<dyn Fn(&str) -> Result<String, Error> + Send + Sync>;
pub type Verifier = fn(&str, &str) -> Result<bool, Error>;

fn config<'a>() -> Config<'a> {
  Config {
    variant: Variant::Argon2id,
    version: Version::Version13,
    mem_cost: 65536,
    time_cost: 10,
    lanes: 4,
    thread_mode: ThreadMode::Parallel,
    secret: &[],
    ad: &[],
    hash_length: 32,
  }
}

pub fn make_hasher(salt: &'static str) -> Hasher {
  Box::new(move |password: &str| {
    Ok(hash_encoded(
      password.as_bytes(),
      salt.as_bytes(),
      &config(),
    )?)
  })
}

pub fn verify(password: &str, hash: &str) -> Result<bool, Error> {
  Ok(verify_encoded(hash, password.as_bytes())?)
}
