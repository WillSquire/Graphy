use crate::error::Error;
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool, PooledConnection};

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub struct Db {
  connection_pool: ConnectionPool,
}

impl Db {
  pub fn new(
    db_user: &str,
    db_password: &str,
    db_name: &str,
    db_server: &str,
  ) -> Result<Db, Error> {
    Ok(Db {
      connection_pool: Pool::builder().max_size(15).build(
        ConnectionManager::<PgConnection>::new(format!(
          "postgres://{}:{}@{}/{}",
          db_user, db_password, db_server, db_name
        )),
      )?,
    })
  }

  pub fn connect(&self) -> Result<Connection, Error> {
    Ok(self.connection_pool.clone().get()?)
  }
}
