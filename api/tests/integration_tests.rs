extern crate api;
extern crate diesel;
extern crate serde_json;
extern crate uuid;

use api::{hasher::Hasher, models::user::User, models::user::UserCreate, tokeniser::Tokeniser};
use serde_json::Value;
use std::str;
use uuid::Uuid;
mod common;

#[test]
fn it_graphiql() {
  let config = common::config();
  let server = common::server(&config);

  let res = warp::test::request().method("GET").path("/").reply(&server);

  assert_eq!(res.status(), 200);
}

#[test]
fn it_create_user() {
  let config = common::config();
  common::db(&config);
  let tokeniser = Tokeniser::new(&config.token_secret);
  let server = common::server(&config);

  let res = warp::test::request()
    .header("content-type", "application/json")
    .method("POST")
    .path("/graphql")
    .body(
      r#"
      {  
        "query": "mutation($user: UserCreate!) {createUser (user: $user)}",
        "variables": {  
            "user": {  
              "id": "00000000-0000-0000-0000-000000000001",
              "name": "Test",
              "email": "00000000-0000-0000-0000-000000000001@test.com",
              "password": "test"
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
    Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
  );
}

#[test]
fn it_read_user() {
  let id = Uuid::new_v4();
  let email = format!("{}-test@test.com", id);
  let password = "test";
  let name = "Tester";

  let config = common::config();
  let db = common::db(&config);
  let hasher = Hasher::new(&config.hash_salt);
  let tokeniser = Tokeniser::new(&config.token_secret);
  let server = common::server(&config);
  let token = User::create(
    &db.connect().unwrap(),
    &hasher.generate,
    &tokeniser.generate,
    &UserCreate {
      id,
      email: email.clone(),
      password: password.to_string(),
      name: Some(name.to_string()),
    },
  )
  .unwrap();

  let res = warp::test::request()
    .header("content-type", "application/json")
    .header("authorization", token)
    .method("POST")
    .path("/graphql")
    .body(format!(
      r#"
      {{
        "query": "query ($userId: Uuid!) {{\n  User(userId: $userId) {{\n    id\n    name\n    email\n  }}\n}}\n",
        "variables": {{
          "userId": "{}"
        }}
      }}
      "#, id),
    )
    .reply(&server);
  let json: Value = serde_json::from_str(str::from_utf8(res.body()).unwrap()).unwrap();
  let result_id = Uuid::parse_str(json["data"]["User"]["id"].as_str().unwrap()).unwrap();
  let result_name = json["data"]["User"]["name"].as_str().unwrap();
  let result_email = json["data"]["User"]["email"].as_str().unwrap();

  assert_eq!(res.status(), 200);
  assert_eq!(result_id, id);
  assert_eq!(result_name, name);
  assert_eq!(result_email, email);
}

#[test]
fn it_read_user_unauthenticated() {
  let id = Uuid::new_v4();
  let email = format!("{}-test@test.com", id);
  let password = "test";
  let name = "Tester";

  let config = common::config();
  let db = common::db(&config);
  let hasher = Hasher::new(&config.hash_salt);
  let tokeniser = Tokeniser::new(&config.token_secret);
  let server = common::server(&config);
  User::create(
    &db.connect().unwrap(),
    &hasher.generate,
    &tokeniser.generate,
    &UserCreate {
      id,
      email: email.clone(),
      password: password.to_string(),
      name: Some(name.to_string()),
    },
  )
  .unwrap();

  let res = warp::test::request()
    .header("content-type", "application/json")
    .method("POST")
    .path("/graphql")
    .body(format!(
      r#"
      {{
        "query": "query ($userId: Uuid!) {{\n  User(userId: $userId) {{\n    id\n    name\n    email\n  }}\n}}\n",
        "variables": {{
          "userId": "{}"
        }}
      }}
      "#, id),
    )
    .reply(&server);
  let json: Value = serde_json::from_str(str::from_utf8(res.body()).unwrap()).unwrap();
  let data = &json["data"];
  let error = &json["errors"][0]["message"].as_str().unwrap();

  assert_eq!(res.status(), 200);
  assert!(data.is_null());
  assert_eq!(error, &"Unauthorised - Must be logged in to view users");
}

// TODO: Finish integration tests

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

#[test]
fn it_login_user() {
  let id = Uuid::new_v4();
  let email = format!("{}-test@test.com", id);
  let password = "test";
  let name = "Tester";

  let config = common::config();
  let db = common::db(&config);
  let hasher = Hasher::new(&config.hash_salt);
  let tokeniser = Tokeniser::new(&config.token_secret);
  let server = common::server(&config);
  User::create(
    &db.connect().unwrap(),
    &hasher.generate,
    &tokeniser.generate,
    &UserCreate {
      id,
      email: email.clone(),
      password: password.to_string(),
      name: Some(name.to_string()),
    },
  )
  .unwrap();

  let res = warp::test::request()
    .header("content-type", "application/json")
    .method("POST")
    .path("/graphql")
    .body(format!(
      r#"
      {{
        "query": "mutation ($user: UserLogin!) {{\n  login(user: $user)\n}}\n",
        "variables": {{
          "user": {{ 
            "email": "{}",
            "password": "{}"
          }}
        }}
      }}
      "#,
      email, password
    ))
    .reply(&server);

  let json: Value = serde_json::from_str(str::from_utf8(res.body()).unwrap()).unwrap();
  let token = &json["data"]["login"].as_str().unwrap();
  let claims = (tokeniser.verify)(token).unwrap();

  assert_eq!(res.status(), 200);
  assert_eq!(claims.sub, id);
}

// #[test]
// fn it_login_user_unauthenticated() { assert_eq!(false, true); }
