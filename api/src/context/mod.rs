use crate::db::Db;
use crate::hasher::{HashVerifier, Hasher};
use crate::tokeniser::{TokenVerifier, Tokeniser};
use juniper::Context as JuniperContext;

pub struct Context {
  pub db: Db,
  pub hasher: Hasher,
  pub hash_verify: HashVerifier,
  pub tokeniser: Tokeniser,
  pub token_verify: TokenVerifier,
}

impl<'a> JuniperContext for Context {}
