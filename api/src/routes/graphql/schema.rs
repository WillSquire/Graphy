use crate::context::Context;
use crate::error::Error;
use crate::models::user::{User, UserCreate, UserEdit, UserLogin};
use juniper::{FieldResult, RootNode};
use uuid::Uuid;

pub struct Query;

graphql_object!(Query: Context |&self| {
  field getUser(&executor, id: Uuid) -> FieldResult<User> {
    Ok(User::read(&executor.context().db.connect()?, &id)?)
  }
});

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
  field createUser(&executor, user: UserCreate) -> FieldResult<String> {
    Ok(User::create(
      &executor.context().db.connect()?,
      &executor.context().hasher.generate,
      &executor.context().tokeniser.generate,
      &user
    )?)
  }

  field updateUser(&executor, user: UserEdit) -> FieldResult<bool> {
    let admin = &executor.context().user.ok_or(Error::Str("Unauthorised - Must be logged in to update user"))?;
    
    Ok(User::update(
      &executor.context().db.connect()?,
      &executor.context().hasher.generate,
      &admin,
      &user
    )?)
  }

  field deleteUser(&executor, id: Uuid) -> FieldResult<bool> {
    let admin = &executor.context().user.ok_or(Error::Str("Unauthorised - Must be logged in to delete user"))?;

    Ok(User::delete(&executor.context().db.connect()?, &admin, &id)?)
  }
  
  field login(&executor, user: UserLogin) -> FieldResult<String> {
    Ok(User::login(
      &executor.context().db.connect()?,
      &executor.context().hasher.verify,
      &executor.context().tokeniser.generate,
      &user
    )?)
  }
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn new() -> Schema {
  Schema::new(Query, Mutation)
}
