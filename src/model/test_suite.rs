use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestSuite {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "test_suite"]
pub struct TestSuiteNew<'a> {
    pub name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestSuiteJson {
    pub name: String,
}
