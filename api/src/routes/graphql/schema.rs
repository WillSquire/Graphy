use crate::context::Context;
use crate::models::user::{User, UserEdit};
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
  field createUser(&executor, user: UserEdit) -> FieldResult<bool> {
    Ok(User::create(&executor.context().db.connect()?, &user)?)
  }

  field updateUser(&executor, user: UserEdit) -> FieldResult<bool> {
    Ok(User::update(&executor.context().db.connect()?, &user)?)
  }

  field deleteUser(&executor, id: Uuid) -> FieldResult<bool> {
    Ok(User::delete(&executor.context().db.connect()?, &id)?)
  }
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn new() -> Schema {
  Schema::new(Query, Mutation)
}
