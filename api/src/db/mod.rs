use crate::error::Error;
use diesel::{pg::PgConnection, r2d2::ConnectionManager, r2d2::CustomizeConnection};
use r2d2::{Pool, PooledConnection};

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
embed_migrations!();

pub struct Db {
  connection_pool: ConnectionPool,
}

impl Db {
  //  Creates a new instance of `Db` and
  /// runs any new migrations. If `testing`
  /// is passed then any database writes
  /// will not be written.
  pub fn new(
    db_user: &str,
    db_password: &str,
    db_name: &str,
    db_server: &str,
    testing: bool,
  ) -> Result<Db, Error> {
    let manager = ConnectionManager::<PgConnection>::new(format!(
      "postgres://{}:{}@{}/{}",
      db_user, db_password, db_server, db_name
    ));

    let connection_pool = if testing {
      Pool::builder()
        .connection_customizer(Box::new(TestTransaction))
        .max_size(15)
        .build(manager)?
    } else {
      Pool::builder().max_size(15).build(manager)?
    };

    embedded_migrations::run(&connection_pool.clone().get()?)?;

    Ok(Db { connection_pool })
  }

  pub fn connect(&self) -> Result<Connection, Error> {
    Ok(self.connection_pool.clone().get()?)
  }
}

#[derive(Debug)]
struct TestTransaction;

impl CustomizeConnection<PgConnection, diesel::r2d2::Error> for TestTransaction {
  fn on_acquire(&self, conn: &mut PgConnection) -> ::std::result::Result<(), diesel::r2d2::Error> {
    use diesel::Connection;

    conn.begin_test_transaction().unwrap();
    Ok(())
  }
}
