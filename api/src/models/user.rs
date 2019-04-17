use crate::db::Connection;
use crate::error::Error;
use crate::hasher::Hasher;
use crate::models::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

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
pub struct UserEdit {
  pub id: Uuid,
  pub email: Option<String>,
  pub password: Option<String>,
  pub name: Option<String>,
}

impl User {
  pub fn create(connection: &Connection, hash: &Hasher, user: &UserCreate) -> Result<bool, Error> {
    Ok(
      diesel::insert_into(users::table)
        .values(UserCreate {
          password: hash(&user.password)?,
          ..user.clone()
        })
        .execute(connection)?
        > 0,
    )
  }

  pub fn read(connection: &Connection, id: &Uuid) -> Result<User, Error> {
    Ok(
      users::table
        .select((users::id, users::email, users::name))
        .find(id)
        .first::<User>(connection)?,
    )
  }

  pub fn update(connection: &Connection, hash: &Hasher, user: &UserEdit) -> Result<bool, Error> {
    let mut user_update = user.clone();

    if user_update.password.is_some() {
      user_update.password = Some(hash(&user_update.password.unwrap())?)
    }

    Ok(diesel::update(user).set(user_update).execute(connection)? > 0)
  }

  pub fn delete(connection: &Connection, id: &Uuid) -> Result<bool, Error> {
    Ok(diesel::delete(users::table.find(id)).execute(connection)? > 0)
  }
}
