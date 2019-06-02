use crate::db::Connection;
use crate::error::Error;
use crate::models::group::Group;
use crate::models::schema::{groups, users_groups};
use crate::models::user::User;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Associations, GraphQLObject, Identifiable, Queryable)]
#[belongs_to(User)]
#[belongs_to(Group)]
#[table_name = "users_groups"]
pub struct UserGroup {
  pub id: Uuid,
  pub added_at: DateTime<Utc>,
  pub user_id: Uuid,
  pub group_id: Uuid,
}

#[derive(AsChangeset, Associations, GraphQLInputObject, Insertable)]
#[belongs_to(User)]
#[belongs_to(Group)]
#[table_name = "users_groups"]
pub struct UserGroupCreate {
  pub id: Uuid,
  pub added_at: DateTime<Utc>,
  pub user_id: Uuid,
  pub group_id: Uuid,
}

impl UserGroup {
  pub fn add(
    connection: &Connection,
    user_id: &Uuid,
    group_id: &Uuid,
    added_at: &DateTime<Utc>,
  ) -> Result<bool, Error> {
    Ok(
      diesel::insert_into(users_groups::table)
        .values(UserGroupCreate {
          id: Uuid::new_v4(),
          added_at: *added_at,
          user_id: *user_id,
          group_id: *group_id,
        })
        .execute(connection)?
        == 1,
    )
  }

  pub fn read_groups(connection: &Connection, user_id: &Uuid) -> Result<Vec<Group>, Error> {
    use diesel::pg::expression::dsl::any;

    let user = User::read(connection, user_id, user_id)?;
    let group_ids = UserGroup::belonging_to(&user).select(users_groups::group_id);

    Ok(
      groups::table
        .filter(groups::id.eq(any(group_ids)))
        .load::<Group>(connection)?,
    )
  }
}
