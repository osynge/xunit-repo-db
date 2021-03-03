use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestFile {
    pub id: i32,
    pub directory: String,
    pub file_name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "test_file"]
pub struct TestFileNew<'a> {
    pub directory: &'a str,
    pub file_name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestFileJson {
    pub directory: String,
    pub file_name: String,
}
