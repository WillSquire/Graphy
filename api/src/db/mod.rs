use crate::error::Error;
use diesel::{
  r2d2::ConnectionManager, r2d2::CustomizeConnection, sql_query, Connection as Diesel_Connection,
  PgConnection, RunQueryDsl,
};
use r2d2::{Pool, PooledConnection};

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
embed_migrations!();

pub struct Db {
  connection_pool: ConnectionPool,
}

impl Db {
  /// Creates a new instance of `Db` and
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
    let connection_url = format!(
      "postgres://{}:{}@{}/{}",
      db_user, db_password, db_server, db_name
    );

    // Attempts to create the database on connection error.
    if PgConnection::establish(&connection_url).is_err() {
      let conn = PgConnection::establish(&format!(
        "postgres://{}:{}@{}",
        db_user, db_password, db_server
      ))
      .expect(&format!("Failed to connect to database \"{}\".", db_name));
      sql_query(format!("CREATE DATABASE {}", db_name))
        .execute(&conn)
        .expect(&format!("Failed to create database \"{}\".", db_name));
    }

    let manager = ConnectionManager::<PgConnection>::new(connection_url);

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
