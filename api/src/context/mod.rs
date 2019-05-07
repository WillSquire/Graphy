use crate::db::Db;
use crate::hasher::Hasher;
use crate::tokeniser::Tokeniser;
use juniper::Context as JuniperContext;
use std::sync::Arc;
use uuid::Uuid;

pub struct Context {
  pub db: Arc<Db>,
  pub hasher: Arc<Hasher>,
  pub tokeniser: Arc<Tokeniser>,
  pub user: Option<Uuid>,
}

impl<'a> JuniperContext for Context {}
