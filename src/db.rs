use crate::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
embed_migrations!();
pub fn run_migrations(conn: &SqliteConnection) {
    let _ = diesel_migrations::run_pending_migrations(&*conn);
}

#[cfg(test)]

pub fn establish_connection() -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");

    run_migrations(&pool.get().unwrap());
    println!("dfsdf");
    pool
}

#[cfg(not(test))]
pub fn establish_connection() -> Pool {
    dotenv::dotenv().ok();
    println!("elephant");
    let database_url = std::env::var("DATABASE_URL").expect("Database not found");
    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .expect("Failed to create DB pool.");
    database_pool
}
