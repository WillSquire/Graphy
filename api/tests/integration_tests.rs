extern crate api;
extern crate serde_json;
extern crate uuid;

use serde_json::Value;
use std::str;
mod common;

#[test]
fn it_graphiql() {
  let config = common::config();
  let server = common::server(config);

  let res = warp::test::request().method("GET").path("/").reply(&server);

  assert_eq!(res.status(), 200);
}

#[test]
fn it_create_user() {
  let config = common::config();
  let tokeniser = api::tokeniser::Tokeniser::new(config.token_secret.clone());
  let server = common::server(config);

  let res = warp::test::request()
    .header("content-type", "application/json")
    .method("POST")
    .path("/graphql")
    .body(
      r#"
      {  
        "query":"mutation($user: UserCreate!) {createUser (user: $user)}",
        "variables":{  
            "user":{  
              "id":"00000000-0000-0000-0000-000000000001",
              "name":"Test",
              "email":"00000000-0000-0000-0000-000000000001@test.com",
              "password":"test"
            }
        }
      }
      "#,
    )
    .reply(&server);
  let json: Value = serde_json::from_str(str::from_utf8(res.body()).unwrap()).unwrap();
  let token = &json["data"]["createUser"].as_str().unwrap();
  let claims = (tokeniser.verify)(token).unwrap();

  assert_eq!(res.status(), 200);
  assert_eq!(
    claims.sub,
    uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
  );
}

// TODO: Finish integration tests

// #[test]
// fn it_read_user() { assert_eq!(false, true); }

// #[test]
// fn it_read_user_unauthenticated() { assert_eq!(false, true); }

// #[test]
// fn it_read_user_unauthorized() { assert_eq!(false, true); }

// #[test]
// fn it_update_user() { assert_eq!(false, true); }

// #[test]
// fn it_update_user_unauthenticated() { assert_eq!(false, true); }

// #[test]
// fn it_update_user_unauthorized() { assert_eq!(false, true); }

// #[test]
// fn it_delete_user() { assert_eq!(false, true); }

// #[test]
// fn it_delete_user_unauthenticated() { assert_eq!(false, true); }

// #[test]
// fn it_delete_user_unauthorized() { assert_eq!(false, true); }

// #[test]
// fn it_login_user() { assert_eq!(false, true); }

// #[test]
// fn it_login_user_unauthenticated() { assert_eq!(false, true); }

// #[test]
// fn it_login_user_unauthorized() { assert_eq!(false, true); }
