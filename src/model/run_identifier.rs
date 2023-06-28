use crate::model::project::Project;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[diesel(table_name = run_identifier)]
#[belongs_to(Project, foreign_key = "fk_project")]
pub struct RunIdentifier {
    pub id: i32,
    pub sk: String,
    pub client_identifier: String,
    pub created: i64,
    pub fk_project: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = run_identifier)]
pub struct RunIdentifierNew<'a> {
    pub sk: &'a str,
    pub client_identifier: &'a str,
    pub created: i64,
    pub fk_project: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunIdentifierJson {
    pub sk: Option<String>,
    pub client_identifier: Option<String>,
    pub created: Option<i64>,
}
