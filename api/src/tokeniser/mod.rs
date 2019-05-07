use crate::error::Error;
use chrono::{Duration, Utc};
use jwt::{decode, encode, Header, Validation};
use uuid::Uuid;

pub type Tokenise = Box<dyn Fn(Uuid) -> Result<String, Error> + Send + Sync>;
pub type VerifyToken = Box<dyn Fn(&str) -> Result<Claims, Error> + Send + Sync>;

pub struct Tokeniser {
  pub tokenise: Tokenise,
  pub verify: VerifyToken,
}

impl Tokeniser {
  pub fn new(secret: String) -> Tokeniser {
    let iss = env!("CARGO_PKG_NAME").to_string();
    let secret2 = secret.clone(); // Can't `move` twice, don't want `Arc`
    let validation_config = Validation {
      iss: Some(iss.clone()),
      ..Default::default()
    };

    Tokeniser {
      tokenise: Box::new(move |user_id: Uuid| {
        let iat = Utc::now();
        let exp = iat + Duration::minutes(15);
        let jti = Uuid::new_v4();

        Ok(encode(
          &Header::default(),
          &Claims {
            exp: exp.timestamp(),
            iat: iat.timestamp(),
            iss: iss.clone(),
            jti,
            sub: user_id,
          },
          secret.as_bytes(),
        )?)
      }),
      verify: Box::new(move |token: &str| {
        Ok(decode::<Claims>(token, secret2.as_bytes(), &validation_config)?.claims)
      }),
    }
  }
}

/// Claims for Json Web Token (JWT):
///
/// - Expiry (`exp`): When the token expires.
/// - Issued at (`iat`): When the token was issued.
/// - Issuer (`iss`): Verifies the service that issued the token.
/// - Json web token ID (`jti`): Useful for blacklisting issued tokens.
/// - Subject (`sub`): Unique subject identifier of the token, in this case the `User` UUID.
#[derive(Deserialize, Serialize)]
pub struct Claims {
  pub exp: i64,
  pub iat: i64,
  pub iss: String,
  pub jti: Uuid,
  pub sub: Uuid,
}
