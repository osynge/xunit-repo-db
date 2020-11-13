use crate::model::test_case_skipped::{TestCaseSkipped, TestCaseSkippedJson, TestCaseSkippedNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case_skipped(
    pool: web::Data<Pool>,
    filter_fk_test_run: i32,
    item: &TestCaseSkippedJson,
) -> Result<TestCaseSkipped, diesel::result::Error> {
    use crate::schema::test_case_skipped::dsl::*;
    let db_connection = pool.get().unwrap();
    match test_case_skipped
        .filter(name.eq(&item.name))
        .filter(classname.eq(&item.classname))
        .filter(time.eq(item.time))
        .filter(skipped_message.eq(&item.skipped_message))
        .filter(fk_test_run.eq(filter_fk_test_run))
        .first::<TestCaseSkipped>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = TestCaseSkippedNew {
                name: &item.name,
                classname: &item.classname,
                time: item.time,
                skipped_message: item.skipped_message.as_deref(),
                fk_test_run: filter_fk_test_run,
            };

            insert_into(test_case_skipped)
                .values(&new_keyvalue)
                .execute(&db_connection)
                .expect("Error saving new test case skipped");

            let result = test_case_skipped
                .order(id.desc())
                .first(&db_connection)
                .unwrap();
            Ok(result)
        }
    }
}
