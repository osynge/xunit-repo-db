use crate::schema::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Project {
    pub id: i32,
    pub sk: String,
    pub identiifier: String,
    pub human_name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "project"]
pub struct ProjectNew<'a> {
    pub sk: &'a str,
    pub identiifier: &'a str,
    pub human_name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectJson {
    pub sk: String,
    pub identiifier: String,
    pub human_name: String,
}
