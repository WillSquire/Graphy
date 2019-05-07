pub mod schema;

use crate::context::Context;
use crate::db::Db;
use crate::hasher::Hasher;
use crate::tokeniser::Tokeniser;
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

pub fn context(
  db: Arc<Db>,
  hasher: Arc<Hasher>,
  tokeniser: Arc<Tokeniser>,
) -> BoxedFilter<(Context,)> {
  let db = warp::any().map(move || db.clone());
  let hasher = warp::any().map(move || hasher.clone());
  let tokeniser = warp::any().map(move || tokeniser.clone());

  warp::any()
    .and(db)
    .and(hasher)
    .and(tokeniser)
    .and(warp::header::optional::<String>("authorization"))
    .and_then(
      |db: Arc<Db>, hasher: Arc<Hasher>, tokeniser: Arc<Tokeniser>, auth_header: Option<String>| {
        let mut user = None;

        if let Some(token) = auth_header {
          match (tokeniser.verify)(&token) {
            Ok(claims) => user = Some(claims.sub),
            _ => return Err(warp::reject::not_found()), // TODO Return UNAUTHORIZED
          }
        }

        Ok(Context {
          db,
          hasher,
          tokeniser,
          user,
        })
      },
    )
    .boxed()
}

pub fn graphql(context: BoxedFilter<(Context,)>) -> BoxedFilter<(warp::http::Response<Vec<u8>>,)> {
  juniper_warp::make_graphql_filter(schema::new(), context)
}
