use crate::model::test_case;
use crate::model::test_file_run;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "test_case_skipped"]
#[belongs_to(test_case::TestCase, foreign_key = "fk_test_case")]
#[belongs_to(test_file_run::TestFileRun, foreign_key = "fk_test_file_run")]
pub struct TestCaseSkipped {
    pub id: i32,
    pub fk_test_case: i32,
    pub time: Option<f32>,
    pub skipped_message: Option<String>,
    pub fk_test_file_run: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case_skipped"]
pub struct TestCaseSkippedNew<'a> {
    pub fk_test_case: i32,
    pub time: Option<&'a f32>,
    pub skipped_message: Option<&'a str>,
    pub fk_test_file_run: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseSkippedJson {
    pub fk_test_case: i32,
    pub time: Option<f32>,
    pub skipped_message: Option<String>,
}
