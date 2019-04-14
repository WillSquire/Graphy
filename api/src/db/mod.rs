use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use crate::error::Error;

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub struct Db {
    connection_pool: ConnectionPool,
}

impl Db {
    pub fn new(
        db_user: String,
        db_password: String,
        db_name: String,
        db_server: String,
    ) -> Result<Db, Error> {
        Ok(Db {
            connection_pool: Pool::builder().max_size(15).build(ConnectionManager::<
                PgConnection,
            >::new(format!(
                "postgres://{}:{}@{}/{}",
                db_user, db_password, db_server, db_name
            )))?,
        })
    }

    pub fn connect(&self) -> Result<Connection, Error> {
        Ok(self.connection_pool.clone().get()?)
    }
}