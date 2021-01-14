use crate::model::test_case;
use crate::model::test_file_run;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "test_case_pass"]
#[belongs_to(test_case::TestCase, foreign_key = "fk_test_case")]
#[belongs_to(test_file_run::TestFileRun, foreign_key = "fk_test_file_run")]
pub struct TestCasePass {
    pub id: i32,
    pub fk_test_case: i32,
    pub time: Option<f32>,
    pub fk_test_file_run: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case_pass"]
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
