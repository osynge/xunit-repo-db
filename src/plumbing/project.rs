use crate::model::project::{Project, ProjectJson, ProjectNew};
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;

fn project_get_by_sk_identiifier_humanname(
    conn: &DbConnection,
    filter_sk: &String,
    filter_identiifier: &String,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(sk.eq(filter_sk))
        .filter(identiifier.eq(filter_identiifier))
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(conn)
}

fn project_get_by_identiifier_humanname(
    conn: &DbConnection,
    filter_identiifier: &String,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(identiifier.eq(filter_identiifier))
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(conn)
}

fn project_get_by_sk_identiifier(
    conn: &DbConnection,
    filter_sk: &String,
    filter_identiifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(sk.eq(filter_sk))
        .filter(identiifier.eq(filter_identiifier))
        .first::<Project>(conn)
}

fn project_get_by_sk_humanname(
    conn: &DbConnection,
    filter_sk: &String,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(sk.eq(filter_sk))
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(conn)
}

fn project_get_by_humanname(
    conn: &DbConnection,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(conn)
}

fn project_get_by_identiifier(
    conn: &DbConnection,
    filter_identiifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(identiifier.eq(filter_identiifier))
        .first::<Project>(conn)
}

fn project_get_by_sk(
    conn: &DbConnection,
    filter_sk: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project.filter(sk.eq(filter_sk)).first::<Project>(conn)
}

fn project_insert_sk_identiifier_humanname(
    conn: &DbConnection,
    insert_sk: &String,
    insert_identiifier: &String,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;

    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_identiifier,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_identiifier_humanname(
    conn: &DbConnection,
    insert_identiifier: &String,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let insert_sk = Uuid::new_v4().to_string();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_identiifier,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_sk_humanname(
    conn: &DbConnection,
    insert_sk: &String,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_human_name,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_sk_identiifier(
    conn: &DbConnection,
    insert_sk: &String,
    insert_identiifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;

    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_identiifier,
        human_name: &insert_identiifier,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");

    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_humanname(
    conn: &DbConnection,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let insert_sk = Uuid::new_v4().to_string();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_human_name,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_identiifier(
    conn: &DbConnection,
    insert_identiifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let insert_sk = Uuid::new_v4().to_string();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identiifier: &insert_identiifier,
        human_name: &insert_identiifier,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");

    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

pub fn add_project(
    conn: &DbConnection,
    sk: Option<&String>,
    identiifier: Option<&String>,
    human_name: Option<&String>,
) -> Result<Project, diesel::result::Error> {
    match (sk, identiifier, human_name) {
        (Some(sk), Some(identiifier), Some(human_name)) => {
            match project_get_by_sk_identiifier_humanname(conn, &sk, &identiifier, &human_name) {
                Ok(p) => Ok(p),
                Err(_) => {
                    project_insert_sk_identiifier_humanname(conn, &sk, &identiifier, &human_name)
                }
            }
        }
        (None, Some(identiifier), Some(human_name)) => {
            match project_get_by_identiifier_humanname(conn, &identiifier, &human_name) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_identiifier_humanname(conn, &identiifier, &human_name),
            }
        }
        (Some(sk), None, Some(human_name)) => {
            match project_get_by_sk_humanname(conn, &sk, &human_name) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_sk_humanname(conn, &sk, &human_name),
            }
        }
        (Some(sk), Some(identiifier), None) => {
            match project_get_by_sk_identiifier(conn, &sk, &identiifier) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_sk_identiifier(conn, &sk, &identiifier),
            }
        }
        (None, None, Some(human_name)) => match project_get_by_humanname(conn, &human_name) {
            Ok(p) => Ok(p),
            Err(_) => project_insert_humanname(conn, &human_name),
        },
        (None, Some(identiifier), None) => match project_get_by_identiifier(conn, identiifier) {
            Ok(p) => Ok(p),
            Err(_) => project_insert_identiifier(conn, identiifier),
        },
        (None, None, None) => Err(diesel::result::Error::NotFound),
        (Some(sk), None, None) => project_get_by_sk(conn, &sk),
    }
}
