use crate::db::Connection;
use crate::error::Error;
use crate::models::schema::{groups, users_groups};
use crate::models::user::User;
use crate::models::user_group::UserGroup;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(GraphQLObject, Identifiable, Queryable)]
#[table_name = "groups"]
pub struct Group {
  pub id: Uuid,
  pub name: String,
  pub created_at: DateTime<Utc>,
}

#[derive(AsChangeset, GraphQLInputObject, Insertable)]
#[table_name = "groups"]
pub struct GroupCreate {
  pub id: Uuid,
  pub name: String,
  pub created_at: DateTime<Utc>,
}

#[derive(AsChangeset, GraphQLInputObject, Identifiable, Insertable)]
#[table_name = "groups"]
pub struct GroupUpdate {
  pub id: Uuid,
  pub name: Option<String>,
}

impl Group {
  pub fn create(
    connection: &Connection,
    user_id: &Uuid,
    group: &GroupCreate,
  ) -> Result<bool, Error> {
    let group_created = diesel::insert_into(groups::table)
      .values(group)
      .execute(connection)?
      == 1;
    let user_added = UserGroup::add(connection, user_id, &group.id, &group.created_at)?;

    Ok(group_created && user_added)
  }

  pub fn read(connection: &Connection, user_id: &Uuid, group_id: &Uuid) -> Result<Group, Error> {
    use diesel::pg::expression::dsl::any;

    let user = User::read(connection, user_id, user_id)?;
    let group_ids = UserGroup::belonging_to(&user).select(users_groups::group_id);

    Ok(
      groups::table
        .filter(groups::id.eq(any(group_ids)))
        .find(group_id)
        .first::<Group>(connection)?,
    )
  }

  pub fn read_all(connection: &Connection, user_id: &Uuid) -> Result<Vec<Group>, Error> {
    use diesel::pg::expression::dsl::any;

    let user = User::read(connection, user_id, user_id)?;
    let group_ids = UserGroup::belonging_to(&user).select(users_groups::group_id);

    Ok(
      groups::table
        .filter(groups::id.eq(any(group_ids)))
        .load::<Group>(connection)?,
    )
  }

  pub fn update(
    connection: &Connection,
    user_id: &Uuid,
    group: &GroupUpdate,
  ) -> Result<bool, Error> {
    use diesel::pg::expression::dsl::any;

    let user = User::read(connection, user_id, user_id)?;
    let group_ids = UserGroup::belonging_to(&user).select(users_groups::group_id);

    Ok(
      diesel::update(group)
        .set(group)
        .filter(groups::id.eq(any(group_ids)))
        .execute(connection)?
        == 1,
    )
  }

  pub fn delete(connection: &Connection, user_id: &Uuid, group_id: &Uuid) -> Result<bool, Error> {
    use diesel::pg::expression::dsl::any;

    let user = User::read(connection, user_id, user_id)?;
    let group_ids = UserGroup::belonging_to(&user).select(users_groups::group_id);

    Ok(
      diesel::delete(
        groups::table
          .filter(groups::id.eq(any(group_ids)))
          .find(group_id),
      )
      .execute(connection)?
        == 1,
    )
  }
}
