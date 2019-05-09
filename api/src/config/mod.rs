use crate::error::Error;
use clap::{App, Arg, ArgGroup};
use std::fs;

// Todo: Add validators (i.e. min length for token & salt, etc)

pub struct Config {
  pub db_name: String,
  pub db_user: String,
  pub db_password: String,
  pub db_server: String,
  pub hash_salt: String,
  pub token_secret: String,
}

impl Config {
  pub fn from_args() -> Result<Config, Error> {
    let args = App::new(env!("CARGO_PKG_NAME"))
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about(env!("CARGO_PKG_DESCRIPTION"))
      .bin_name(env!("CARGO_PKG_NAME"))
      .arg(
        Arg::with_name("db-name")
          .short("d")
          .long("db-name")
          .value_name("NAME")
          .help("Sets database name")
          .takes_value(true),
      )
      .arg(
        Arg::with_name("db-name-file")
          .long("db-name-file")
          .value_name("FILE")
          .help("Sets database name via file")
          .takes_value(true),
      )
      .group(
        ArgGroup::with_name("database-name")
          .args(&["db-name", "db-name-file"])
          .required(true),
      )
      .arg(
        Arg::with_name("db-user")
          .short("u")
          .long("db-user")
          .value_name("USERNAME")
          .help("Sets database username")
          .takes_value(true),
      )
      .arg(
        Arg::with_name("db-user-file")
          .long("db-user-file")
          .value_name("FILE")
          .help("Sets database username via file")
          .takes_value(true),
      )
      .group(
        ArgGroup::with_name("database-user")
          .args(&["db-user", "db-user-file"])
          .required(true),
      )
      .arg(
        Arg::with_name("db-password")
          .short("p")
          .long("db-password")
          .value_name("PASSWORD")
          .help("Sets database password")
          .takes_value(true),
      )
      .arg(
        Arg::with_name("db-password-file")
          .long("db-password-file")
          .value_name("FILE")
          .help("Sets database password via file")
          .takes_value(true),
      )
      .group(
        ArgGroup::with_name("database-password")
          .args(&["db-password", "db-password-file"])
          .required(true),
      )
      .arg(
        Arg::with_name("db-server")
          .short("s")
          .long("db-server")
          .value_name("SERVER")
          .help("Sets database server")
          .takes_value(true)
          .default_value("127.0.0.1"),
      )
      .arg(
        Arg::with_name("hash-salt")
          .long("hash-salt")
          .value_name("SALT")
          .help("Sets hash salt")
          .takes_value(true),
      )
      .arg(
        Arg::with_name("hash-salt-file")
          .long("hash-salt-file")
          .value_name("FILE")
          .help("Sets hash salt via file")
          .takes_value(true),
      )
      .group(
        ArgGroup::with_name("hasher-salt")
          .args(&["hash-salt", "hash-salt-file"])
          .required(true),
      )
      .arg(
        Arg::with_name("token-secret")
          .long("token-secret")
          .value_name("SECRET")
          .help("Sets token secret")
          .takes_value(true),
      )
      .arg(
        Arg::with_name("token-secret-file")
          .long("token-secret-file")
          .value_name("FILE")
          .help("Sets token secret via file")
          .takes_value(true),
      )
      .group(
        ArgGroup::with_name("tokeniser-secret")
          .args(&["token-secret", "token-secret-file"])
          .required(true),
      )
      .get_matches();

    let find_arg = |val, file| -> Result<String, Error> {
      if let Some(val_arg) = args.value_of(val) {
        return Ok(val_arg.to_string());
      };

      if let Some(file_arg) = args.value_of(file) {
        return Ok(fs::read_to_string(file_arg)?.trim().to_string());
      };

      Err(Error::Str("Args missing"))
    };

    Ok(Config {
      db_name: find_arg("db-name", "db-name-file")?,
      db_user: find_arg("db-user", "db-user-file")?,
      db_password: find_arg("db-password", "db-password-file")?,
      db_server: args.value_of("db-server").unwrap().to_string(),
      hash_salt: find_arg("hash-salt", "hash-salt-file")?,
      token_secret: find_arg("token-secret", "token-secret-file")?,
    })
  }
}
