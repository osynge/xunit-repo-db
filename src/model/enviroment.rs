use crate::model::keyvalue::KeyValueJson;
use crate::model::project;
use crate::schema::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "enviroment"]
#[belongs_to(project::Project, foreign_key = "fk_project")]
pub struct Enviroment {
    pub id: i32,
    pub sk: String,
    pub hash_keyvalue: String,
    pub best_before: Option<i32>,
    pub fk_project: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "enviroment"]
pub struct EnviromentNew<'a> {
    pub sk: &'a str,
    pub hash_keyvalue: &'a str,
    pub best_before: Option<i32>,
    pub fk_project: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnviromentJson {
    pub sk: Option<String>,
    pub key_value: Option<HashMap<String, String>>,
}
