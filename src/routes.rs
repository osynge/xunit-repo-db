use crate::model::project::{Project, ProjectJson, ProjectNew};
use crate::Pool;

use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

pub async fn add_link(
    pool: web::Data<Pool>,
    item: web::Json<ProjectJson>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_project(pool, item))
        .await
        .map(|project| HttpResponse::Created().json(project))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn project_get_by_sk_identiifier_humanname(
    pool: web::Data<Pool>,
    filter_sk: &String,
    filter_identiifier: &String,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    project
        .filter(sk.eq(filter_sk))
        .filter(identiifier.eq(filter_identiifier))
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(&db_connection)
}

fn project_get_by_identiifier_humanname(
    pool: web::Data<Pool>,
    filter_identiifier: &String,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    project
        .filter(identiifier.eq(filter_identiifier))
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(&db_connection)
}

fn project_get_by_sk_identiifier(
    pool: web::Data<Pool>,
    filter_sk: &String,
    filter_identiifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    project
        .filter(sk.eq(filter_sk))
        .filter(identiifier.eq(filter_identiifier))
        .first::<Project>(&db_connection)
}

fn project_get_by_sk_humanname(
    pool: web::Data<Pool>,
    filter_sk: &String,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    project
        .filter(sk.eq(filter_sk))
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(&db_connection)
}

fn project_get_by_humanname(
    pool: web::Data<Pool>,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    project
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(&db_connection)
}

fn project_get_by_identiifier(
    pool: web::Data<Pool>,
    filter_identiifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    project
        .filter(identiifier.eq(filter_identiifier))
        .first::<Project>(&db_connection)
}

fn project_get_by_sk(
    pool: web::Data<Pool>,
    filter_sk: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    project
        .filter(sk.eq(filter_sk))
        .first::<Project>(&db_connection)
}

fn project_insert_sk_identiifier_humanname(
    pool: web::Data<Pool>,
    insert_sk: &String,
    insert_identiifier: &String,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();

    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_identiifier,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(&db_connection)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(&db_connection).unwrap();
    Ok(result)
}

fn project_insert_identiifier_humanname(
    pool: web::Data<Pool>,
    insert_identiifier: &String,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    let insert_sk = Uuid::new_v4().to_string();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_identiifier,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(&db_connection)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(&db_connection).unwrap();
    Ok(result)
}

fn project_insert_sk_humanname(
    pool: web::Data<Pool>,
    insert_sk: &String,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_human_name,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(&db_connection)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(&db_connection).unwrap();
    Ok(result)
}

fn project_insert_sk_identiifier(
    pool: web::Data<Pool>,
    insert_sk: &String,
    insert_identiifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();

    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_identiifier,
        human_name: &insert_identiifier,
    };
    insert_into(project)
        .values(&new_link)
        .execute(&db_connection)
        .expect("Error saving new project");

    let result = project.order(id.desc()).first(&db_connection).unwrap();
    Ok(result)
}

fn project_insert_humanname(
    pool: web::Data<Pool>,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    let insert_sk = Uuid::new_v4().to_string();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_human_name,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(&db_connection)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(&db_connection).unwrap();
    Ok(result)
}

fn project_insert_identiifier(
    pool: web::Data<Pool>,
    insert_identiifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let db_connection = pool.get().unwrap();
    let insert_sk = Uuid::new_v4().to_string();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_identiifier,
        human_name: &insert_identiifier,
    };
    insert_into(project)
        .values(&new_link)
        .execute(&db_connection)
        .expect("Error saving new project");

    let result = project.order(id.desc()).first(&db_connection).unwrap();
    Ok(result)
}

fn add_project(
    pool: web::Data<Pool>,
    item: web::Json<ProjectJson>,
) -> Result<Project, diesel::result::Error> {
    match (&item.sk, &item.identiifier, &item.human_name) {
        (Some(sk), Some(identiifier), Some(human_name)) => {
            match project_get_by_sk_identiifier_humanname(
                pool.clone(),
                &sk,
                &identiifier,
                &human_name,
            ) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_sk_identiifier_humanname(
                    pool.clone(),
                    &sk,
                    &identiifier,
                    &human_name,
                ),
            }
        }
        (None, Some(identiifier), Some(human_name)) => {
            match project_get_by_identiifier_humanname(pool.clone(), &identiifier, &human_name) {
                Ok(p) => Ok(p),
                Err(_) => {
                    project_insert_identiifier_humanname(pool.clone(), &identiifier, &human_name)
                }
            }
        }
        (Some(sk), None, Some(human_name)) => {
            match project_get_by_sk_humanname(pool.clone(), &sk, &human_name) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_sk_humanname(pool.clone(), &sk, &human_name),
            }
        }
        (Some(sk), Some(identiifier), None) => {
            match project_get_by_sk_identiifier(pool.clone(), &sk, &identiifier) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_sk_identiifier(pool.clone(), &sk, &identiifier),
            }
        }
        (None, None, Some(human_name)) => match project_get_by_humanname(pool.clone(), &human_name)
        {
            Ok(p) => Ok(p),
            Err(_) => project_insert_humanname(pool.clone(), &human_name),
        },
        (None, Some(identiifier), None) => {
            match project_get_by_identiifier(pool.clone(), &identiifier) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_identiifier(pool.clone(), &identiifier),
            }
        }
        (None, None, None) => Err(diesel::result::Error::NotFound),
        (Some(sk), None, None) => project_get_by_sk(pool.clone(), &sk),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;
    use actix_web::test;
    use actix_web::{http::header, web, App};
    use diesel::r2d2::ConnectionManager;
    use diesel::SqliteConnection;

    #[actix_rt::test]
    async fn test_index() {
        let database_url = "foo.db";
        let database_pool = Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(database_url))
            .unwrap();
        let mut app = test::init_service(
            App::new()
                .data(database_pool.clone())
                .route("/homessss", web::post().to(home)),
        )
        .await;
        let ti = r#"{ "sk": "mykey", "identiifier": "identiifier2", "human_name" : "sdfsdfsf" }"#;

        //let req = test::TestRequest::post().uri("/").to_request();
        let req = test::TestRequest::post()
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(ti)
            .uri("/homessss")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        //assert!(resp.status().is_client_error());

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
    /*
    #[actix_rt::test]
    async fn test_index_post() {
        let database_url = "foo.db";
        let database_pool = Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(database_url))
            .unwrap();
        let mut app = test::init_service(
            App::new()
                .data(database_pool.clone())
                .route("/addlink", web::post().to(add_link_noop)),
        )
        .await;
        let ti = r#"{ "sk": "mykey", "identiifier": "identiifier2", "human_name" : "sdfsdfsf" }"#
            .as_bytes();
        //let req = test::TestRequest::post().uri("/").to_request();
        let req = test::TestRequest::post()
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(ti)
            .uri("/addlink")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        //assert!(resp.status().is_client_error());

        assert_eq!(resp.status(), 201);
    }
    */
}
