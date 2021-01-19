use crate::model::test_run::{TestRun, TestRunNew};
use crate::DbConnection;
use chrono::Utc;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_run(
    conn: &DbConnection,
    new_fk_run_identifier: i32,
    new_fk_enviroment: i32,
) -> Result<TestRun, diesel::result::Error> {
    use crate::schema::test_run::dsl::*;
    match test_run
        .filter(fk_run_identifier.eq(new_fk_run_identifier))
        .filter(fk_enviroment.eq(new_fk_enviroment))
        .first::<TestRun>(conn)
    {
        Ok(p) => return Ok(p),
        Err(_) => {
            let new_created = Utc::now().timestamp();
            let new_test_run_new = TestRunNew {
                fk_run_identifier: new_fk_run_identifier,
                fk_enviroment: new_fk_enviroment,
                created: new_created,
            };
            insert_into(test_run)
                .values(&new_test_run_new)
                .execute(conn)
                .expect("Error saving new test_run_new");

            let result = test_run.order(id.desc()).first(conn)?;
            Ok(result)
        }
    }
}
