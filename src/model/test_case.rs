use crate::schema::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestCase {
    pub id: i32,
    pub name: String,
    pub fk_test_class : i32,
    pub fk_test_suite : i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case"]
pub struct TestCaseNew<'a> {
    pub name: &'a str,
    pub fk_test_class : i32,
    pub fk_test_suite : i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseJson {
    pub name: String,
    pub fk_test_class : i32,
    pub fk_test_suite : i32,
}
