use crate::model::test_file;
use crate::model::test_run;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "test_file_run"]
#[belongs_to(test_file::TestFile, foreign_key = "fk_test_file")]
#[belongs_to(test_run::TestRun, foreign_key = "fk_test_run")]
pub struct TestFileRun {
    pub id: i32,
    pub sk: String,
    pub fk_test_file: i32,
    pub fk_test_run: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_file_run"]
pub struct TestFileRunNew<'a> {
    pub sk: &'a str,
    pub fk_test_file: i32,
    pub fk_test_run: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestFileRunJson {
    pub sk: String,
    pub fk_test_file: i32,
    pub fk_test_run: i32,
}
