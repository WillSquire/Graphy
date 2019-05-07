use crate::error::Error;
use argon2::{hash_encoded, verify_encoded, Config, ThreadMode, Variant, Version};

pub type Hash = Box<dyn Fn(&str) -> Result<String, Error> + Send + Sync>;
pub type VerifyHash = Box<dyn Fn(&str, &str) -> Result<bool, Error> + Send + Sync>;

pub struct Hasher {
  pub hash: Hash,
  pub verify: VerifyHash,
}

impl Hasher {
  pub fn new(salt: String) -> Hasher {
    let config = Config {
      variant: Variant::Argon2id,
      version: Version::Version13,
      mem_cost: 4096,
      time_cost: 8,
      lanes: 6,
      thread_mode: ThreadMode::Parallel,
      secret: &[],
      ad: &[],
      hash_length: 32,
    };
    
    Hasher {
      hash: Box::new(move |password: &str|
        Ok(hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?)
      ),
      verify: Box::new(move |hash: &str, password: &str|
        Ok(verify_encoded(hash, password.as_bytes())?)
      ),
    }
  }
}
