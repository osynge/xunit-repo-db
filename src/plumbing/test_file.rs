use crate::model::test_file::{TestFile, TestFileNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_file(
    pool: web::Data<Pool>,
    new_directory: &String,
    new_file_name: &String,
) -> Result<TestFile, diesel::result::Error> {
    use crate::schema::test_file::dsl::*;
    let db_connection = pool.get().unwrap();
    match test_file
        .filter(directory.eq(new_directory))
        .filter(file_name.eq(new_file_name))
        .first::<TestFile>(&db_connection)
    {
        Ok(p) => return Ok(p),
        Err(_) => {
            let new_test_file_new = TestFileNew {
                directory: new_directory.as_str(),
                file_name: new_file_name.as_str(),
            };
            insert_into(test_file)
                .values(&new_test_file_new)
                .execute(&db_connection)
                .expect("Error saving new test_file_new");

            let result = test_file.order(id.desc()).first(&db_connection)?;
            Ok(result)
        }
    }
}
