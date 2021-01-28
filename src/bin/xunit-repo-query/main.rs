#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate xunit_repo;
mod routes;
mod plumbing;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use xunit_repo::db;
use xunit_repo::model;
use xunit_repo::schema;

pub type DbConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_pool = db::establish_connection();
    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .route("/", web::get().to(routes::home))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
