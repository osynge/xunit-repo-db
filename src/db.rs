use crate::schema::*;
use crate::Pool;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use url::Url;

embed_migrations!();
pub fn run_migrations(conn: &SqliteConnection) {
    embedded_migrations::run(conn);
}

pub fn sql_lite_establish_connection_mem() -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let database_pool = Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");

    run_migrations(&database_pool.get().unwrap());
    database_pool
}

pub fn sql_lite_establish_connection_file(database_path: &str, create_db: bool) -> Pool {
    if create_db {
        let connection = SqliteConnection::establish(&database_path).unwrap();
        run_migrations(&connection);
    };
    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_path))
        .expect("Failed to create DB pool.");
    run_migrations(&database_pool.get().unwrap());
    database_pool
}
pub fn establish_connection_pool(input: &str, create_db: bool) -> Pool {
    let dburl = Url::parse(input).unwrap();
    match dburl.scheme() {
        "memory" => sql_lite_establish_connection_mem(),
        "sqlite" => sql_lite_establish_connection_file(dburl.path(), create_db),
        _ => {
            panic!("url scheme is invalid.");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tempdir;
    #[test]
    fn establish_connection_in_mem() {
        establish_connection_pool("memory://", true).get().unwrap();
    }
    #[test]
    fn establish_connection_on_disk() {
        let tmp_dir =
            tempdir::TempDir::new("establish_connection_on_disk").expect("create temp dir");
        let file_path = tmp_dir.path().join("sqlite.db");
        let url = format!("sqlite://{}", file_path.to_str().unwrap());
        let foo = establish_connection_pool(&url, true).get().unwrap();
        foo.begin_test_transaction();
    }
}
