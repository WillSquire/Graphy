use api::config::Config;
use std::fs;

pub fn config() -> Config {
  // TODO: Temporary fix. Try to pass these as `args`
  Config::new(
    fs::read_to_string("../secrets/db_name.txt").unwrap().trim(),
    fs::read_to_string("../secrets/db_user.txt").unwrap().trim(),
    fs::read_to_string("../secrets/db_password.txt")
      .unwrap()
      .trim(),
    "127.0.0.1",
    fs::read_to_string("../secrets/hash_salt.txt")
      .unwrap()
      .trim(),
    true,
    fs::read_to_string("../secrets/token_secret.txt")
      .unwrap()
      .trim(),
  )
}

pub fn server(
  config: Config,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> {
  api::server(config).unwrap()
}
