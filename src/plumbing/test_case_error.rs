use crate::model::test_case_error::{TestCaseError, TestCaseErrorJson, TestCaseErrorNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case_error(
    pool: web::Data<Pool>,
    filter_fk_test_run: i32,
    item: &TestCaseErrorJson,
) -> Result<TestCaseError, diesel::result::Error> {
    use crate::schema::test_case_error::dsl::*;
    let db_connection = pool.get().unwrap();
    match test_case_error
        .filter(name.eq(&item.name))
        .filter(classname.eq(&item.classname))
        .filter(time.eq(item.time))
        .filter(error_message.eq(&item.error_message))
        .filter(error_type.eq(&item.error_type))
        .filter(error_description.eq(&item.error_description))
        .filter(system_out.eq(&item.system_out))
        .filter(system_err.eq(&item.system_err))
        .filter(fk_test_run.eq(filter_fk_test_run))
        .first::<TestCaseError>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = TestCaseErrorNew {
                name: &item.name,
                classname: &item.classname,
                time: item.time,
                error_message: item.error_message.as_deref(),
                error_type: item.error_type.as_deref(),
                error_description: item.error_description.as_deref(),
                system_out: item.system_out.as_deref(),
                system_err: item.system_err.as_deref(),
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
