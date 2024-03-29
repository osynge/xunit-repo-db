use crate::model::test_class::TestClass;
use crate::model::test_suite::TestSuite;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[diesel(table_name = test_case)]
#[diesel(belongs_to(TestClass, foreign_key = fk_test_case_class))]
#[diesel(belongs_to(TestSuite, foreign_key = fk_test_suite))]
pub struct TestCase {
    pub id: i32,
    pub sk: String,
    pub name: String,
    pub fk_test_case_class: i32,
    pub fk_test_suite: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = test_case)]
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
