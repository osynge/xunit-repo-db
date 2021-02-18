use crate::schema::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestClass {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case_class"]
pub struct TestClassNew<'a> {
    pub name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestClassJson {
    pub name: String,
}
