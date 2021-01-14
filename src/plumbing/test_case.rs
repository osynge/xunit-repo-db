use crate::model::test_case::{TestCase, TestCaseNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case(
    pool: web::Data<Pool>,
    new_name: &String,
    new_classname: &String,
) -> Result<TestCase, diesel::result::Error> {
    use crate::schema::test_case::dsl::*;
    let db_connection = pool.get().unwrap();
    match test_case
        .filter(name.eq(new_name))
        .filter(classname.eq(new_classname))
        .first::<TestCase>(&db_connection)
    {
        Ok(p) => return Ok(p),
        Err(_) => {
            let new_test_case_new = TestCaseNew {
                name: new_name.as_str(),
                classname: new_classname.as_str(),
            };
            insert_into(test_case)
                .values(&new_test_case_new)
                .execute(&db_connection)
                .expect("Error saving new test_case_new");

            let result = test_case.order(id.desc()).first(&db_connection)?;
            Ok(result)
        }
    }
}
