use crate::model::test_case_failure::{TestCaseFailure, TestCaseFailureJson, TestCaseFailureNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case_failure(
    pool: web::Data<Pool>,
    filter_fk_test_run: i32,
    item: &TestCaseFailureJson,
) -> Result<TestCaseFailure, diesel::result::Error> {
    use crate::schema::test_case_failure::dsl::*;
    let db_connection = pool.get().unwrap();
    match test_case_failure
        .filter(name.eq(&item.name))
        .filter(classname.eq(&item.classname))
        .filter(time.eq(item.time))
        .filter(failure_message.eq(&item.failure_message))
        .filter(failure_type.eq(&item.failure_type))
        .filter(failure_description.eq(&item.failure_description))
        .filter(system_out.eq(&item.system_out))
        .filter(system_err.eq(&item.system_err))
        .filter(fk_test_run.eq(filter_fk_test_run))
        .first::<TestCaseFailure>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = TestCaseFailureNew {
                name: &item.name,
                classname: &item.classname,
                time: item.time,
                failure_message: item.failure_message.as_deref(),
                failure_type: item.failure_type.as_deref(),
                failure_description: item.failure_description.as_deref(),
                system_out: item.system_out.as_deref(),
                system_err: item.system_err.as_deref(),
                fk_test_run: filter_fk_test_run,
            };

            insert_into(test_case_failure)
                .values(&new_keyvalue)
                .execute(&db_connection)
                .expect("Error saving new test case failure");

            let result = test_case_failure
                .order(id.desc())
                .first(&db_connection)
                .unwrap();
            Ok(result)
        }
    }
}
