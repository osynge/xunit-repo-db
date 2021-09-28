#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate url;
use thiserror::Error;
#[cfg(all(feature = "sqlite", not(feature = "postgres")))]
mod db;
#[cfg(all(not(feature = "sqlite"), feature = "postgres"))]
mod db_postgres;

pub mod model;
pub mod schema;
use diesel::r2d2::{self, ConnectionManager};

#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("sqlite and postgres are mutually exclusive and cannot be enabled together");
#[cfg(all(not(feature = "sqlite"), not(feature = "postgres")))]
compile_error!("either sqlite of postgres features must be enabled. ");

#[cfg(all(feature = "sqlite", not(feature = "postgres")))]
pub type DbConnection = diesel::SqliteConnection;

#[cfg(all(not(feature = "sqlite"), feature = "postgres"))]
pub type DbConnection = diesel::pg::PgConnection;

#[cfg(all(not(feature = "sqlite"), not(feature = "postgres")))]
pub struct DbConnection {}

pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("url parsing error")]
    UrlParseError(#[from] url::ParseError),
    #[error("io parsing error")]
    IoErr(#[from] std::io::Error),
    #[error("File not found '{0}'.")]
    FilePathIsNotFile(String),
    #[error("Db connection scheme is invalid '{0}'.")]
    InvalidConnectionScheme(String),
    #[error("Db connection host is invalid '{0}'.")]
    InvalidConnectionHost(String),
}

pub fn establish_connection_pool(input: &str, create_db: bool) -> Result<Pool, ConnectionError> {
    #[cfg(all(feature = "sqlite", not(feature = "postgres")))]
    return db::establish_connection_pool(input, create_db);
    #[cfg(all(not(feature = "sqlite"), feature = "postgres"))]
    return db_postgres::establish_connection_pool(input, create_db);
    Err(ConnectionError::FilePathIsNotFile("".to_string()))
}
