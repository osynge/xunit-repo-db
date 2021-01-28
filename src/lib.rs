#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod db;
pub mod model;
pub mod schema;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type DbConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;
