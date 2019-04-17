use argon2;
use diesel;
use r2d2;
use std::fmt;
use std::io;

/// ### Error
/// Error handler type for the program.
/// All errors should return this error
/// type and bubble up.
#[derive(Debug)]
pub enum Error {
  Hasher(argon2::Error),
  Diesel(diesel::result::Error),
  Io(io::Error),
  R2d2(r2d2::Error),
  Str(&'static str),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Hasher(ref err) => err.fmt(f),
      Error::Diesel(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
      Error::R2d2(ref err) => err.fmt(f),
      Error::Str(ref err) => err.fmt(f),
    }
  }
}

impl From<argon2::Error> for Error {
  fn from(err: argon2::Error) -> Self {
    Error::Hasher(err)
  }
}

impl From<diesel::result::Error> for Error {
  fn from(err: diesel::result::Error) -> Self {
    Error::Diesel(err)
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
