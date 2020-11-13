use crate::model::test_run;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "test_case_pass"]
#[belongs_to(test_run::TestRun, foreign_key = "fk_test_run")]
pub struct TestCasePass {
    pub id: i32,
    pub name: String,
    pub classname: String,
    pub time: Option<i32>,
    pub fk_test_run: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case_pass"]
pub struct TestCasePassNew<'a> {
    pub name: &'a str,
    pub classname: &'a str,
    pub time: Option<i32>,
    pub fk_test_run: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCasePassJson {
    pub name: String,
    pub classname: String,
    pub time: Option<i32>,
}
