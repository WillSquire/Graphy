use crate::error::Error;
use argon2::{hash_encoded, verify_encoded, Config, ThreadMode, Variant, Version};

pub type Hasher = Box<dyn Fn(&str) -> Result<String, Error> + Send + Sync>;
pub type HashVerifier = fn(&str, &str) -> Result<bool, Error>;

fn config<'a>() -> Config<'a> {
  Config {
    variant: Variant::Argon2id,
    version: Version::Version13,
    mem_cost: 4096,
    time_cost: 8,
    lanes: 6,
    thread_mode: ThreadMode::Parallel,
    secret: &[],
    ad: &[],
    hash_length: 32,
  }
}

pub fn make_hasher(salt: String) -> Hasher {
  let config = config();
  Box::new(move |password: &str| Ok(hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?))
}

pub fn hash_verify(hash: &str, password: &str) -> Result<bool, Error> {
  Ok(verify_encoded(hash, password.as_bytes())?)
}
