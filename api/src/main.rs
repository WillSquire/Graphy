#![feature(decl_macro, proc_macro_hygiene)]
#![feature(trait_alias)]
#![feature(fnbox)]

extern crate argon2;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate juniper;
extern crate clap;
extern crate juniper_rocket;
extern crate r2d2;
#[macro_use]
extern crate serde;
extern crate uuid;

mod config;
mod context;
mod db;
mod error;
mod hasher;
mod models;
mod routes;
mod tokeniser;

use config::Config;
use context::Context;
use db::Db;
use error::Error;
use hasher::{hash_verify, make_hasher};
use tokeniser::{make_token_verifier, make_tokeniser};

fn main() -> Result<(), Error> {
  let config = Config::new()?;
  let db = Db::new(
    &config.db_user,
    &config.db_password,
    &config.db_name,
    &config.db_server,
  )?;
  let hasher = make_hasher(config.hash_salt); // Todo: Take from `config`
  let tokeniser = make_tokeniser("change_me"); // Todo: Take from `config`
  let token_verify = make_token_verifier("change_me"); // Todo: Take from `config`

  rocket::ignite()
    .manage(Context {
      db,
      hasher,
      hash_verify,
      tokeniser,
      token_verify,
    })
    .manage(routes::graphql::schema::new())
    .mount(
      "/",
      routes![
        routes::graphql::graphiql,
        routes::graphql::get_graphql_handler,
        routes::graphql::post_graphql_handler
      ],
    )
    .launch();

  Ok(())
}
