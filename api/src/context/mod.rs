use juniper::{Context as JuniperContext};
use crate::db::Db;

pub struct Context {
  pub db: Db,
}

impl JuniperContext for Context {}
