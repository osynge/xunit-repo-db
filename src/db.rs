use crate::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use url::Url;
embed_migrations!();
pub fn run_migrations(conn: &SqliteConnection) {
    let _ = diesel_migrations::run_pending_migrations(&*conn);
}

pub fn sql_lite_establish_connection_mem() -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");

    run_migrations(&pool.get().unwrap());
    println!("dfsdf");
    pool
}

pub fn sql_lite_establish_connection_2(database_path: &str, create_db: bool) -> Pool {
    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_path))
        .expect("Failed to create DB pool.");
    if create_db {
        run_migrations(&database_pool.get().unwrap());
    };
    database_pool
}
pub fn establish_connection(input: &str, create_db: bool) -> Pool {
    let dburl = Url::parse(input).unwrap();
    match dburl.scheme() {
        "memory" => sql_lite_establish_connection_mem(),
        "sqlite" => sql_lite_establish_connection_2(dburl.path(), create_db),
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
        establish_connection("memory://", true);
    }
    #[test]
    fn establish_connection_on_disk() {
        let tmp_dir =
            tempdir::TempDir::new("establish_connection_on_disk").expect("create temp dir");
        let file_path = tmp_dir.path().join("sqlite.db");
        let url = format!("sqlite://{}", file_path.to_str().unwrap());
        establish_connection(&url, true);
    }
}
