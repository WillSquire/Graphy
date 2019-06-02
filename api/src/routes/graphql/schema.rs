use crate::context::Context;
use crate::error::Error;
use crate::models::user::{User, UserCreate, UserUpdate, UserLogin};
use crate::models::group::{Group, GroupCreate, GroupUpdate};
use juniper::{FieldResult, RootNode};
use uuid::Uuid;

pub struct Query;

graphql_object!(Query: Context |&self| {
  field User(&executor, user_id: Uuid) -> FieldResult<User> {
    let admin_id = &executor.context().user.ok_or(Error::Str("Unauthorised - Must be logged in to view users"))?;

    Ok(User::read(&executor.context().db.connect()?, &admin_id, &user_id)?)
  }

  field Group(&executor, group_id: Uuid) -> FieldResult<Group> {
    let user_id = &executor.context().user.ok_or(Error::Str("Unauthorised - Must be logged in to view groups"))?;

    Ok(Group::read(&executor.context().db.connect()?, &user_id, &group_id)?)
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

  field updateUser(&executor, user: UserUpdate) -> FieldResult<bool> {
    let admin = &executor.context().user.ok_or(Error::Str("Unauthorised - Must be logged in to update user"))?;
    
    Ok(User::update(
      &executor.context().db.connect()?,
      &executor.context().hasher.generate,
      &admin,
      &user
    )?)
  }

  field deleteUser(&executor, user_id: Uuid) -> FieldResult<bool> {
    let admin = &executor.context().user.ok_or(Error::Str("Unauthorised - Must be logged in to delete user"))?;

    Ok(User::delete(&executor.context().db.connect()?, &admin, &user_id)?)
  }
  
  field login(&executor, user: UserLogin) -> FieldResult<String> {
    Ok(User::login(
      &executor.context().db.connect()?,
      &executor.context().hasher.verify,
      &executor.context().tokeniser.generate,
      &user
    )?)
  }

  field createGroup(&executor, group: GroupCreate) -> FieldResult<bool> {
    let user_id = &executor.context().user.ok_or(Error::Str("Unauthorised - Must be logged in to create groups"))?;
    
    Ok(Group::create(
      &executor.context().db.connect()?,
      &user_id,
      &group
    )?)
  }

  field updateGroup(&executor, group: GroupUpdate) -> FieldResult<bool> {
    let admin = &executor.context().user.ok_or(Error::Str("Unauthorised - Must be logged in to update group"))?;
    
    Ok(Group::update(
      &executor.context().db.connect()?,
      &admin,
      &group
    )?)
  }

  field deleteGroup(&executor, group_id: Uuid) -> FieldResult<bool> {
    let admin = &executor.context().user.ok_or(Error::Str("Unauthorised - Must be logged in to delete user"))?;

    Ok(Group::delete(&executor.context().db.connect()?, &admin, &group_id)?)
  }
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn new() -> Schema {
  Schema::new(Query, Mutation)
}
