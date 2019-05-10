extern crate argon2;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate juniper;
extern crate clap;
extern crate juniper_warp;
extern crate r2d2;
#[macro_use]
extern crate serde;
extern crate uuid;
extern crate warp;

pub mod config;
mod context;
mod db;
pub mod error;
mod hasher;
mod models;
mod routes;
mod tokeniser;

use config::Config;
use db::Db;
use error::Error;
use hasher::Hasher;
use routes::graphql::{context, graphql};
use std::sync::Arc;
use tokeniser::Tokeniser;
use warp::Filter;

pub fn run(config: Config) -> Result<(), Error> {
  let db = Arc::new(Db::new(
    &config.db_user,
    &config.db_password,
    &config.db_name,
    &config.db_server,
  )?);
  let hasher = Arc::new(Hasher::new(config.hash_salt));
  let tokeniser = Arc::new(Tokeniser::new(config.token_secret));

  let log = warp::log("warp_server");

  warp::serve(
    warp::get2()
      .and(warp::path::end().and(juniper_warp::graphiql_filter("/graphql")))
      .or(warp::path("graphql").and(graphql(context(db, hasher, tokeniser))))
      .with(log),
  )
  .run((config.address, 8000));

  Ok(())
}
