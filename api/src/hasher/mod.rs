use crate::error::Error;
use argon2::{hash_encoded, verify_encoded, Config, ThreadMode, Variant, Version};

pub type HashGenerator = Box<dyn Fn(&str) -> Result<String, Error> + Send + Sync>;
pub type HashVerifier = Box<dyn Fn(&str, &str) -> Result<bool, Error> + Send + Sync>;

pub struct Hasher {
  pub generate: HashGenerator,
  pub verify: HashVerifier,
}

impl Hasher {
  /// Creates a new `Hasher` instance.
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
      generate: Box::new(move |password: &str| {
        Ok(hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?)
      }),
      verify: Box::new(move |hash: &str, password: &str| {
        Ok(verify_encoded(hash, password.as_bytes())?)
      }),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_hash() {
    let salt = "somesalt";
    let password = "password";

    let hasher = Hasher::new(salt.to_string());
    let hash = (hasher.generate)(password);

    assert_eq!(hash.is_ok(), true);
  }

  #[test]
  fn test_verify_hash() {
    let salt = "somesalt";
    let password = "password";

    let hasher = Hasher::new(salt.to_string());
    let hash = &(hasher.generate)(password).unwrap();
    let verified_password = (hasher.verify)(hash, password);

    assert_eq!(verified_password.is_ok(), true);
  }
}
