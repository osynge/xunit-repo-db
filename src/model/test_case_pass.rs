use crate::model::test_case::TestCase;
use crate::model::test_file_run::TestFileRun;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[diesel(table_name = test_case_pass)]
#[belongs_to(TestCase, foreign_key = "fk_test_case")]
#[belongs_to(TestFileRun, foreign_key = "fk_test_file_run")]
pub struct TestCasePass {
    pub id: i32,
    pub fk_test_case: i32,
    pub time: Option<f32>,
    pub fk_test_file_run: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = test_case_pass)]
pub struct TestCasePassNew {
    pub fk_test_case: i32,
    pub time: Option<f32>,
    pub fk_test_file_run: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCasePassJson {
    pub fk_test_case: i32,
    pub time: Option<f32>,
}
