use crate::model::test_run;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "test_case_skipped"]
#[belongs_to(test_run::TestRun, foreign_key = "fk_test_run")]
pub struct TestCaseSkipped {
    pub id: i32,
    pub name: String,
    pub classname: String,
    pub time: Option<f32>,
    pub skipped_message: Option<String>,
    pub fk_test_run: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case_skipped"]
pub struct TestCaseSkippedNew<'a> {
    pub name: &'a str,
    pub classname: &'a str,
    pub time: Option<f32>,
    pub skipped_message: Option<&'a str>,
    pub fk_test_run: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseSkippedJson {
    pub name: String,
    pub classname: String,
    pub time: Option<f32>,
    pub skipped_message: Option<String>,
}
