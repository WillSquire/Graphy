use juniper::{Context as JuniperContext};
use crate::db::Db;
use crate::hasher::{Hasher, Verifier};

pub struct Context {
  pub db: Db,
  pub hasher: Hasher,
  pub verifier: Verifier
}

impl <'a> JuniperContext for Context {}
