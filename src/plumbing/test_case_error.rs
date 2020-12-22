use crate::model::test_case_error::{TestCaseError, TestCaseErrorJson, TestCaseErrorNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case_error(
    pool: web::Data<Pool>,
    filter_fk_test_run: i32,
    tc_name: &String,
    tc_classname: &String,
    tc_time: Option<f32>,
    tc_error_message: Option<&String>,
    tc_error_type: Option<&String>,
    tc_error_description: Option<&String>,
    tc_system_out: Option<&String>,
    tc_system_err: Option<&String>,
) -> Result<TestCaseError, diesel::result::Error> {
    use crate::schema::test_case_error::dsl::*;
    let db_connection = pool.get().unwrap();
    match test_case_error
        .filter(name.eq(tc_name))
        .filter(classname.eq(tc_classname))
        .filter(time.eq(tc_time))
        .filter(error_message.eq(tc_error_message))
        .filter(error_type.eq(tc_error_type))
        .filter(error_description.eq(tc_error_description))
        .filter(system_out.eq(tc_system_out))
        .filter(system_err.eq(tc_system_err))
        .filter(fk_test_run.eq(filter_fk_test_run))
        .first::<TestCaseError>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = TestCaseErrorNew {
                name: tc_name,
                classname: tc_classname,
                time: tc_time,
                error_message: tc_error_message.map(|s| s.as_str()),
                error_type: tc_error_type.map(|s| s.as_str()),
                error_description: tc_error_description.map(|s| s.as_str()),
                system_out: tc_system_out.map(|s| s.as_str()),
                system_err: tc_system_err.map(|s| s.as_str()),
                fk_test_run: filter_fk_test_run,
            };

            insert_into(test_case_error)
                .values(&new_keyvalue)
                .execute(&db_connection)
                .expect("Error saving new test case error");

            let result = test_case_error
                .order(id.desc())
                .first(&db_connection)
                .unwrap();
            Ok(result)
        }
    }
}
