#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate url;
use thiserror::Error;
mod db_postgres;

pub mod model;
pub mod schema;
use diesel::r2d2::{self, ConnectionManager};

pub type DbConnection = diesel::pg::PgConnection;

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
    db_postgres::establish_connection_pool(input, create_db)
}
