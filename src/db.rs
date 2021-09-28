use crate::schema::*;
use crate::ConnectionError;
use crate::Pool;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use thiserror::Error;
use url::{Host, Url};

embed_migrations!();
pub fn run_migrations(conn: &SqliteConnection) {
    embedded_migrations::run(conn);
}

#[derive(Debug, PartialEq)]
enum DbConnectParams {
    SqlLiteMemory,
    SqlLitePath(String),
}

fn parse_connection_url_sqlite(connection_url: &Url) -> Result<DbConnectParams, ConnectionError> {
    match connection_url.host() {
        Some(p) => match p {
            Host::Domain(d) => match d {
                "memory" => Ok(DbConnectParams::SqlLiteMemory),
                _ => Err(ConnectionError::InvalidConnectionHost(d.to_string())),
            },
            Host::Ipv4(a) => Err(ConnectionError::InvalidConnectionHost(a.to_string())),
            Host::Ipv6(a) => Err(ConnectionError::InvalidConnectionHost(a.to_string())),
        },
        None => Ok(DbConnectParams::SqlLitePath(
            connection_url.path().to_string(),
        )),
    }
}

fn parse_connection_url(url: &str) -> Result<DbConnectParams, ConnectionError> {
    let parsed_url = Url::parse(url)?;
    match parsed_url.scheme() {
        "sqlite" => parse_connection_url_sqlite(&parsed_url),
        _ => Err(ConnectionError::InvalidConnectionScheme(
            parsed_url.scheme().to_string(),
        )),
    }
}

pub fn sql_lite_establish_connection_mem() -> Result<Pool, ConnectionError> {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let database_pool = Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");

    run_migrations(&database_pool.get().unwrap());
    Ok(database_pool)
}

pub fn sql_lite_establish_connection_file(
    database_path: &str,
    create_db: bool,
) -> Result<Pool, ConnectionError> {
    if create_db {
        let connection = SqliteConnection::establish(&database_path).unwrap();
        run_migrations(&connection);
    };
    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_path))
        .expect("Failed to create DB pool.");
    run_migrations(&database_pool.get().unwrap());
    Ok(database_pool)
}
pub fn establish_connection_pool(input: &str, create_db: bool) -> Result<Pool, ConnectionError> {
    match parse_connection_url(input)? {
        DbConnectParams::SqlLiteMemory => Ok(sql_lite_establish_connection_mem()?),
        DbConnectParams::SqlLitePath(path) => {
            Ok(sql_lite_establish_connection_file(&path, create_db)?)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tempdir;
    #[test]
    fn parse_url_sqlite_mem() {
        assert!(parse_connection_url("sqlite://memory").unwrap() == DbConnectParams::SqlLiteMemory)
    }
    #[test]
    fn parse_url_sqlite_relative_file() {
        assert!(
            parse_connection_url("sqlite:relative.db").unwrap()
                == DbConnectParams::SqlLitePath("relative.db".to_string())
        )
    }

    #[test]
    fn parse_url_sqlite_absolute_file() {
        assert!(
            parse_connection_url("sqlite:/tmp/absolute.db").unwrap()
                == DbConnectParams::SqlLitePath("/tmp/absolute.db".to_string())
        )
    }

    #[test]
    fn establish_connection_in_mem() {
        let foo = establish_connection_pool("sqlite://memory", true)
            .unwrap()
            .get()
            .unwrap();
    }
    #[test]
    fn establish_connection_on_disk() {
        let tmp_dir =
            tempdir::TempDir::new("establish_connection_on_disk").expect("create temp dir");
        let file_path = tmp_dir.path().join("sqlite.db");
        let url = format!("sqlite://{}", file_path.to_str().unwrap());
        let foo = establish_connection_pool(&url, true)
            .unwrap()
            .get()
            .unwrap();
        foo.begin_test_transaction();
    }
}
