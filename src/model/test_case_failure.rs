use crate::model::test_case::TestCase;
use crate::model::test_file_run::TestFileRun;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "test_case_failure"]
#[belongs_to(TestCase, foreign_key = "fk_test_case")]
#[belongs_to(TestFileRun, foreign_key = "fk_test_file_run")]
pub struct TestCaseFailure {
    pub id: i32,
    pub fk_test_case: i32,
    pub time: Option<f32>,
    pub failure_message: Option<String>,
    pub failure_type: Option<String>,
    pub failure_description: Option<String>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
    pub fk_test_file_run: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case_failure"]
pub struct TestCaseFailureNew<'a> {
    pub fk_test_case: i32,
    pub time: Option<&'a f32>,
    pub failure_message: Option<&'a str>,
    pub failure_type: Option<&'a str>,
    pub failure_description: Option<&'a str>,
    pub system_out: Option<&'a str>,
    pub system_err: Option<&'a str>,
    pub fk_test_file_run: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseFailureJson {
    pub name: String,
    pub classname: String,
    pub time: Option<f32>,
    pub failure_message: Option<String>,
    pub failure_type: Option<String>,
    pub failure_description: Option<String>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}
