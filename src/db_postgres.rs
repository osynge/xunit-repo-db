use crate::schema::*;
use crate::ConnectionError;
use crate::Pool;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use thiserror::Error;
use url::{Host, Url};

embed_migrations!();
pub fn run_migrations(conn: &PgConnection) {
    embedded_migrations::run(conn);
}

pub fn establish_connection_pool(db_url: &str, create_db: bool) -> Result<Pool, ConnectionError> {
    if create_db {
        let connection = PgConnection::establish(&db_url).unwrap();
        run_migrations(&connection);
    };
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let database_pool = Pool::new(manager).expect("Postgres connection pool could not be created");
    run_migrations(&database_pool.get().unwrap());
    Ok(database_pool)
}
