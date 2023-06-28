use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Project {
    pub id: i32,
    pub sk: String,
    pub identifier: String,
    pub human_name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = project)]
pub struct ProjectNew<'a> {
    pub sk: &'a str,
    pub identifier: &'a str,
    pub human_name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectJson {
    pub sk: Option<String>,
    pub identifier: Option<String>,
    pub human_name: Option<String>,
}

impl From<Project> for ProjectJson {
    fn from(project: Project) -> Self {
        ProjectJson {
            sk: Some(project.sk),
            identifier: Some(project.identifier),
            human_name: Some(project.human_name),
        }
    }
}
