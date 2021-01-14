use crate::model::test_file_run::{TestFileRun, TestFileRunNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub fn add_test_file_run(
    pool: web::Data<Pool>,
    new_fk_test_file: i32,
    new_fk_test_run: i32,
) -> Result<TestFileRun, diesel::result::Error> {
    use crate::schema::test_file_run::dsl::*;
    let db_connection = pool.get().unwrap();
    match test_file_run
        .filter(fk_test_file.eq(new_fk_test_file))
        .filter(fk_test_run.eq(new_fk_test_run))
        .first::<TestFileRun>(&db_connection)
    {
        Ok(p) => return Ok(p),
        Err(_) => {
            let sk_new = Uuid::new_v4().to_string();
            let new_test_file_run = TestFileRunNew {
                sk: sk_new.as_str(),
                fk_test_file: new_fk_test_file,
                fk_test_run: new_fk_test_run,
            };
            insert_into(test_file_run)
                .values(&new_test_file_run)
                .execute(&db_connection)
                .expect("Error saving new test_file_run");

            let result = test_file_run.order(id.desc()).first(&db_connection)?;
            Ok(result)
        }
    }
}
