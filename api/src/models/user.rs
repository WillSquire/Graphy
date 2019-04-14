use crate::db::Connection;
use crate::error::Error;
use crate::models::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(GraphQLObject, Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
  pub id: Uuid,
  pub name: String,
  pub email: String,
}

#[derive(AsChangeset, GraphQLInputObject, Identifiable, Insertable)]
#[table_name = "users"]
pub struct UserEdit {
  pub id: Uuid,
  pub name: Option<String>,
  pub email: Option<String>,
}

impl User {
  pub fn create(connection: &Connection, user: &UserEdit) -> Result<bool, Error> {
    diesel::insert_into(users::table)
      .values(user)
      .execute(connection)?;
    Ok(true)
  }

  pub fn read(connection: &Connection, id: &Uuid) -> Result<User, Error> {
    Ok(users::table.find(id).first::<User>(connection)?)
  }

  pub fn update(connection: &Connection, user: &UserEdit) -> Result<bool, Error> {
    diesel::update(user).set(user).execute(connection)?;
    Ok(true)
  }

  pub fn delete(connection: &Connection, id: &Uuid) -> Result<bool, Error> {
    diesel::delete(users::table.find(id)).execute(connection)?;
    Ok(true)
  }
}
