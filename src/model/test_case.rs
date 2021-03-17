use crate::model::test_class;
use crate::model::test_suite;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "test_case"]
#[belongs_to(test_class::TestClass, foreign_key = "fk_test_case_class")]
#[belongs_to(test_suite::TestSuite, foreign_key = "fk_test_suite")]
pub struct TestCase {
    pub id: i32,
    pub sk: String,
    pub name: String,
    pub fk_test_case_class: i32,
    pub fk_test_suite: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_case"]
pub struct TestCaseNew<'a> {
    pub sk: &'a str,
    pub name: &'a str,
    pub fk_test_case_class: i32,
    pub fk_test_suite: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseJson {
    pub sk: Option<String>,
    pub name: String,
    pub fk_test_case_class: i32,
    pub fk_test_suite: i32,
}
