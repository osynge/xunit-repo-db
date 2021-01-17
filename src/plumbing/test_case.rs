use crate::model::test_case::{TestCase, TestCaseNew};
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case(
    conn: &DbConnection,
    new_name: &String,
    new_classname: &String,
) -> Result<TestCase, diesel::result::Error> {
    use crate::schema::test_case::dsl::*;
    match test_case
        .filter(name.eq(new_name))
        .filter(classname.eq(new_classname))
        .first::<TestCase>(conn)
    {
        Ok(p) => return Ok(p),
        Err(_) => {
            let new_test_case_new = TestCaseNew {
                name: new_name.as_str(),
                classname: new_classname.as_str(),
            };
            insert_into(test_case)
                .values(&new_test_case_new)
                .execute(conn)
                .expect("Error saving new test_case_new");

            let result = test_case.order(id.desc()).first(conn)?;
            Ok(result)
        }
    }
}
