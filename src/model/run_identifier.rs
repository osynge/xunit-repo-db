use crate::model::project;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "run_identifier"]
#[belongs_to(project::Project, foreign_key = "fk_project")]
pub struct RunIdentifier {
    pub id: i32,
    pub client_identifier: String,
    pub created: i32,
    pub fk_project: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "run_identifier"]
pub struct RunIdentifierNew<'a> {
    pub client_identifier: &'a str,
    pub created: i32,
    pub fk_project: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunIdentifierJson {
    pub client_identifier: String,
    pub created: i32,
    pub fk_project: i32,
}
