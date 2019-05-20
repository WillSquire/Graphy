use argon2;
use diesel;
use diesel_migrations;
use jwt;
use r2d2;
use std::fmt;
use std::io;

/// ### Error
/// Error handler type for the program.
/// All errors should return this error
/// type and bubble up.
#[derive(Debug)]
pub enum Error {
  Diesel(diesel::result::Error),
  DieselMigrations(diesel_migrations::RunMigrationsError),
  Hasher(argon2::Error),
  Jwt(jwt::errors::Error),
  Io(io::Error),
  R2d2(r2d2::Error),
  Str(&'static str),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Diesel(ref err) => err.fmt(f),
      Error::DieselMigrations(ref err) => err.fmt(f),
      Error::Hasher(ref err) => err.fmt(f),
      Error::Jwt(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
      Error::R2d2(ref err) => err.fmt(f),
      Error::Str(ref err) => err.fmt(f),
    }
  }
}

impl From<diesel::result::Error> for Error {
  fn from(err: diesel::result::Error) -> Self {
    Error::Diesel(err)
  }
}

impl From<diesel_migrations::RunMigrationsError> for Error {
  fn from(err: diesel_migrations::RunMigrationsError) -> Self {
    Error::DieselMigrations(err)
  }
}

impl From<argon2::Error> for Error {
  fn from(err: argon2::Error) -> Self {
    Error::Hasher(err)
  }
}

impl From<jwt::errors::Error> for Error {
  fn from(err: jwt::errors::Error) -> Self {
    Error::Jwt(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::Io(err)
  }
}

impl From<r2d2::Error> for Error {
  fn from(err: r2d2::Error) -> Self {
    Error::R2d2(err)
  }
}
