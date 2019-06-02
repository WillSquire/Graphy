use crate::db::Connection;
use crate::error::Error;
use crate::hasher::{HashGenerator, HashVerifier};
use crate::models::schema::users;
use crate::tokeniser::TokenGenerator;
use diesel::prelude::*;
use uuid::Uuid;

// Todo: Add validator to create & update fields
// E.g. https://github.com/Keats/validator

#[derive(GraphQLObject, Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
  pub id: Uuid,
  pub email: String,
  pub name: Option<String>,
}

#[derive(AsChangeset, Clone, GraphQLInputObject, Insertable)]
#[table_name = "users"]
pub struct UserCreate {
  pub id: Uuid,
  pub email: String,
  pub password: String,
  pub name: Option<String>,
}

#[derive(AsChangeset, Clone, GraphQLInputObject, Identifiable, Insertable)]
#[table_name = "users"]
pub struct UserUpdate {
  pub id: Uuid,
  pub email: Option<String>,
  pub password: Option<String>,
  pub name: Option<String>,
}

#[derive(GraphQLInputObject, Queryable)]
pub struct UserLogin {
  pub email: String,
  pub password: String,
}

impl User {
  pub fn create(
    connection: &Connection,
    hash: &HashGenerator,
    tokenise: &TokenGenerator,
    user: &UserCreate,
  ) -> Result<String, Error> {
    diesel::insert_into(users::table)
      .values(UserCreate {
        password: hash(&user.password)?,
        ..user.clone()
      })
      .execute(connection)?;
    Ok(tokenise(user.id)?)
  }

  pub fn read(connection: &Connection, admin_id: &Uuid, user_id: &Uuid) -> Result<User, Error> {
    if admin_id != user_id {
      return Err(Error::Str(
        "Unauthorised - Only the given user can view their account",
      ));
    }

    Ok(
      users::table
        .select((users::id, users::email, users::name))
        .find(user_id)
        .first::<User>(connection)?,
    )
  }

  pub fn update(
    connection: &Connection,
    hash: &HashGenerator,
    admin_id: &Uuid,
    user: &UserUpdate,
  ) -> Result<bool, Error> {
    if admin_id != &user.id {
      return Err(Error::Str(
        "Unauthorised - Only the given user can update their account",
      ));
    }

    let mut user_update = user.clone();

    if user_update.password.is_some() {
      user_update.password = Some(hash(&user_update.password.unwrap())?)
    }

    Ok(diesel::update(user).set(user_update).execute(connection)? > 0)
  }

  pub fn delete(connection: &Connection, admin_id: &Uuid, user_id: &Uuid) -> Result<bool, Error> {
    if admin_id != user_id {
      return Err(Error::Str(
        "Unauthorised - Only the given user can delete their account",
      ));
    }

    Ok(diesel::delete(users::table.find(user_id)).execute(connection)? > 0)
  }

  pub fn login(
    connection: &Connection,
    verify: &HashVerifier,
    tokenise: &TokenGenerator,
    user: &UserLogin,
  ) -> Result<String, Error> {
    let (id, password_hash) = users::table
      .filter(users::email.eq(&user.email))
      .select((users::id, users::password))
      .first::<(Uuid, String)>(connection)?;

    if verify(&password_hash, &user.password)? {
      Ok(tokenise(id)?)
    } else {
      Err(Error::Str("Not found"))
    }
  }
}
