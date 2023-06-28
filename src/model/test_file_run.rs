use crate::model::test_file::TestFile;
use crate::model::test_run::TestRun;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[diesel(table_name = test_file_run)]
#[diesel(belongs_to(TestFile, foreign_key = fk_test_file))]
#[diesel(belongs_to(TestRun, foreign_key = fk_test_run))]
pub struct TestFileRun {
    pub id: i32,
    pub sk: String,
    pub fk_test_file: i32,
    pub fk_test_run: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = test_file_run)]
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
