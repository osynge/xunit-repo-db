use crate::DbConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{dsl::insert_into, query_builder::nodes::Identifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xunit_repo::schema::enviroment;

pub fn get_all_enviroments_for_project(
    conn: &DbConnection,
    project_sk: &str,
) -> Result<Vec<String>, diesel::result::Error> {
    let result = crate::schema::enviroment::dsl::enviroment
        .inner_join(crate::schema::project::dsl::project)
        .select(crate::schema::enviroment::dsl::sk)
        .filter(crate::schema::project::dsl::sk.eq(project_sk))
        .load::<String>(conn)?;
    Ok(result)
}

pub fn get_enviroments_details(
    conn: &DbConnection,
    env_sk: &str,
) -> Result<Vec<(String, String)>, diesel::result::Error> {
    let result = crate::schema::bind_enviroment_keyvalue::dsl::bind_enviroment_keyvalue
        .inner_join(crate::schema::enviroment::dsl::enviroment)
        .inner_join(crate::schema::keyvalue::dsl::keyvalue)
        .filter(crate::schema::enviroment::dsl::sk.eq(env_sk))
        .select((
            crate::schema::keyvalue::dsl::key,
            crate::schema::keyvalue::dsl::value,
        ))
        .load::<(String, String)>(conn)?;
    Ok(result)
}
