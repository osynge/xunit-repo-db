use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestCaseClass {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = test_case_class)]
pub struct TestCaseClassNew<'a> {
    pub name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseClassJson {
    pub name: String,
}
