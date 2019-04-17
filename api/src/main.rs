#![feature(decl_macro, proc_macro_hygiene)]
#![feature(trait_alias)]
#![feature(fnbox)]

extern crate argon2;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate juniper;
extern crate clap;
extern crate juniper_rocket;
extern crate r2d2;
extern crate uuid;

mod config;
mod context;
mod db;
mod error;
mod hasher;
mod models;
mod routes;

use config::Config;
use context::Context;
use db::Db;
use error::Error;
use hasher::{make_hasher, verify};

fn main() -> Result<(), Error> {
  let config = Config::new()?;
  let hash = make_hasher("change_me");

  rocket::ignite()
    .manage(Context {
      db: Db::new(
        &config.db_user,
        &config.db_password,
        &config.db_name,
        &config.db_server,
      )?,
      hasher: hash,
      verifier: verify
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
