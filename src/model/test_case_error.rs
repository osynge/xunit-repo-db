use crate::model::test_run;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "test_case_error"]
#[belongs_to(test_run::TestRun, foreign_key = "fk_test_file_run")]
pub struct TestCaseError {
    pub id: i32,
    pub fk_test_case: i32,
    pub time: Option<f32>,
    pub error_message: Option<String>,
    pub error_type: Option<String>,
    pub error_description: Option<String>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
    pub fk_test_file_run: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case_error"]
pub struct TestCaseErrorNew<'a> {
    pub fk_test_case: i32,
    pub time: Option<&'a f32>,
    pub error_message: Option<&'a str>,
    pub error_type: Option<&'a str>,
    pub error_description: Option<&'a str>,
    pub system_out: Option<&'a str>,
    pub system_err: Option<&'a str>,
    pub fk_test_file_run: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseErrorJson {
    pub name: String,
    pub classname: String,
    pub time: Option<f32>,
    pub error_message: Option<String>,
    pub error_type: Option<String>,
    pub error_description: Option<String>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}
