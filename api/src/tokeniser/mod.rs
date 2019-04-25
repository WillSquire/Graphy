use crate::error::Error;
use jwt::{decode, encode, Header, Validation};
use uuid::Uuid;

pub type Tokeniser = Box<dyn Fn(Uuid) -> Result<String, Error> + Send + Sync>;
pub type TokenVerifier = Box<dyn Fn(&str) -> Result<Claims, Error> + Send + Sync>;

#[derive(Deserialize, Serialize)]
pub struct Claims {
  pub user: Uuid,
}

pub fn make_tokeniser(secret: &'static str) -> Tokeniser {
  Box::new(move |user: Uuid| {
    Ok(encode(
      &Header::default(),
      &Claims { user },
      secret.as_ref(),
    )?)
  })
}

pub fn make_token_verifier(secret: &'static str) -> TokenVerifier {
  let validation_config = Validation {
    iss: Some(env!("CARGO_PKG_NAME").to_string()),
    ..Default::default()
  };

  Box::new(move |token: &str| {
    Ok(decode::<Claims>(token, secret.as_ref(), &validation_config)?.claims)
  })
}
