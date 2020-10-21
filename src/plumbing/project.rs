use crate::model::project::{Project, ProjectJson, ProjectNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;

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

pub fn add_project(
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
