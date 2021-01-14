use crate::schema::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestCase {
    pub id: i32,
    pub name: String,
    pub classname: String,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case"]
pub struct TestCaseNew<'a> {
    pub name: &'a str,
    pub classname: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseJson {
    pub name: String,
    pub classname: String,
}
