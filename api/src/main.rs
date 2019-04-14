#![feature(decl_macro, proc_macro_hygiene)]

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
mod models;
mod routes;

use config::Config;
use context::Context;
use db::Db;
use error::Error;

fn main() -> Result<(), Error> {
    let config = Config::new()?;

    rocket::ignite()
        .manage(Context {
            db: Db::new(
                config.db_user,
                config.db_password,
                config.db_name,
                config.db_server,
            )?,
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
