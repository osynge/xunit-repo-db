use crate::model::run_identifier::{RunIdentifier, RunIdentifierJson, RunIdentifierNew};
use crate::Pool;
use actix_web::web;
use chrono::Utc;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;

fn run_identifier_get_by_sk_client_identifier(
    pool: web::Data<Pool>,
    filter_fk_project: i32,
    filter_sk: &String,
    filter_client_identifier: &String,
) -> Result<RunIdentifier, diesel::result::Error> {
    use crate::schema::run_identifier::dsl::*;
    let db_connection = pool.get().unwrap();
    run_identifier
        .filter(sk.eq(filter_sk))
        .filter(fk_project.eq(filter_fk_project))
        .filter(client_identifier.eq(filter_client_identifier))
        .first::<RunIdentifier>(&db_connection)
}

fn run_identifier_get_by_client_identifier(
    pool: web::Data<Pool>,
    filter_fk_project: i32,
    filter_client_identifier: &String,
) -> Result<RunIdentifier, diesel::result::Error> {
    use crate::schema::run_identifier::dsl::*;
    let db_connection = pool.get().unwrap();
    run_identifier
        .filter(client_identifier.eq(filter_client_identifier))
        .filter(fk_project.eq(filter_fk_project))
        .first::<RunIdentifier>(&db_connection)
}

fn run_identifier_get_by_sk(
    pool: web::Data<Pool>,
    filter_fk_project: i32,
    filter_sk: &String,
) -> Result<RunIdentifier, diesel::result::Error> {
    use crate::schema::run_identifier::dsl::*;
    let db_connection = pool.get().unwrap();
    run_identifier
        .filter(sk.eq(filter_sk))
        .filter(fk_project.eq(filter_fk_project))
        .first::<RunIdentifier>(&db_connection)
}

fn insert_run_identifier(
    pool: web::Data<Pool>,
    insert_fk_project: i32,
    insert_created: i64,
    insert_sk: &String,
    insert_client_identifier: &String,
) -> Result<RunIdentifier, diesel::result::Error> {
    use crate::schema::run_identifier::dsl::*;
    let db_connection = pool.get().unwrap();
    let new_runident = RunIdentifierNew {
        sk: &insert_sk,
        client_identifier: insert_client_identifier,
        created: insert_created,
        fk_project: insert_fk_project,
    };
    insert_into(run_identifier)
        .values(&new_runident)
        .execute(&db_connection)
        .expect("Error saving new run_identifier");
    let result = run_identifier.order(id.desc()).first(&db_connection)?;
    Ok(result)
}

pub fn add_run_identifier(
    pool: web::Data<Pool>,
    fk_project: i32,
    item: &RunIdentifierJson,
) -> Result<RunIdentifier, diesel::result::Error> {
    let created = match item.created {
        Some(p) => p,
        None => Utc::now().timestamp(),
    };

    match (&item.sk, &item.client_identifier) {
        (Some(sk), Some(client_identifier)) => {
            match run_identifier_get_by_sk_client_identifier(
                pool.clone(),
                fk_project,
                &sk,
                &client_identifier,
            ) {
                Ok(p) => Ok(p),
                Err(_) => {
                    insert_run_identifier(pool.clone(), fk_project, created, sk, client_identifier)
                }
            }
        }

        (None, Some(client_identifier)) => {
            let sk = Uuid::new_v4().to_string();

            match run_identifier_get_by_client_identifier(
                pool.clone(),
                fk_project,
                &client_identifier,
            ) {
                Ok(p) => Ok(p),
                Err(_) => {
                    insert_run_identifier(pool.clone(), fk_project, created, &sk, client_identifier)
                }
            }
        }
        (None, None) => Err(diesel::result::Error::NotFound),
        (Some(sk), None) => run_identifier_get_by_sk(pool.clone(), fk_project, &sk),
    }
}
