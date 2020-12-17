use crate::model::test_case_pass::{TestCasePass, TestCasePassJson, TestCasePassNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn get_upload(
    pool: web::Data<Pool>,
    item: &xunit_repo_interface::Upload,
) -> Result<u32, diesel::result::Error> {
    println!("got:{:#?}", item);
    Ok(3)
}