use crate::model::test_case_pass::{TestCasePass, TestCasePassJson, TestCasePassNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case_pass(
    pool: web::Data<Pool>,
    filter_fk_test_run: i32,
    item: &TestCasePassJson,
) -> Result<TestCasePass, diesel::result::Error> {
    use crate::schema::test_case_pass::dsl::*;
    let db_connection = pool.get().unwrap();
    match test_case_pass
        .filter(name.eq(&item.name))
        .filter(classname.eq(&item.classname))
        .filter(time.eq(item.time))
        .filter(fk_test_run.eq(filter_fk_test_run))
        .first::<TestCasePass>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = TestCasePassNew {
                name: &item.name,
                classname: &item.classname,
                time: item.time,
                fk_test_run: filter_fk_test_run,
            };

            insert_into(test_case_pass)
                .values(&new_keyvalue)
                .execute(&db_connection)
                .expect("Error saving new test case pass");

            let result = test_case_pass
                .order(id.desc())
                .first(&db_connection)
                .unwrap();
            Ok(result)
        }
    }
}
