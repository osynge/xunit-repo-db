use crate::model::test_case_error::{TestCaseError, TestCaseErrorJson, TestCaseErrorNew};
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case_error(
    conn: &DbConnection,
    filter_fk_test_file_run: i32,
    new_fk_test_case: i32,
    tc_time: Option<f32>,
    tc_error_message: Option<&String>,
    tc_error_type: Option<&String>,
    tc_error_description: Option<&String>,
    tc_system_out: Option<&String>,
    tc_system_err: Option<&String>,
) -> Result<TestCaseError, diesel::result::Error> {
    use crate::schema::test_case_error::dsl::*;
    match test_case_error
        .filter(fk_test_case.eq(new_fk_test_case))
        .filter(time.eq(tc_time))
        .filter(error_message.eq(tc_error_message))
        .filter(error_type.eq(tc_error_type))
        .filter(error_description.eq(tc_error_description))
        .filter(system_out.eq(tc_system_out))
        .filter(system_err.eq(tc_system_err))
        .filter(fk_test_file_run.eq(filter_fk_test_file_run))
        .first::<TestCaseError>(conn)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = TestCaseErrorNew {
                fk_test_case: new_fk_test_case,
                time: tc_time,
                error_message: tc_error_message.map(|s| s.as_str()),
                error_type: tc_error_type.map(|s| s.as_str()),
                error_description: tc_error_description.map(|s| s.as_str()),
                system_out: tc_system_out.map(|s| s.as_str()),
                system_err: tc_system_err.map(|s| s.as_str()),
                fk_test_file_run: filter_fk_test_file_run,
            };

            insert_into(test_case_error)
                .values(&new_keyvalue)
                .execute(conn)
                .expect("Error saving new test case error");

            let result = test_case_error.order(id.desc()).first(conn).unwrap();
            Ok(result)
        }
    }
}
