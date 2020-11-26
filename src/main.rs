#[macro_use]
extern crate diesel;
mod model;
mod plumbing;
mod routes;
mod schema;
use xunit_repo_interface;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Database not found");
    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .route("/", web::get().to(routes::home))
            .route("/project_add", web::post().to(routes::project_add))
            .route("/keyvalue_add", web::post().to(routes::keyvalue_add))
            .route("/enviroment_add", web::post().to(routes::enviroment_add))
            .route("/run_add", web::post().to(routes::run_add))
            .route(
                "/test_case_error_add",
                web::post().to(routes::test_case_error_add),
            )
            .route(
                "/test_case_failure_add",
                web::post().to(routes::test_case_failure_add),
            )
            .route(
                "/test_case_skipped_add",
                web::post().to(routes::test_case_skipped_add),
            )
            .route(
                "/test_case_pass_add",
                web::post().to(routes::test_case_pass_add),
            )
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
