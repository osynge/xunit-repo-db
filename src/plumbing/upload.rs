use crate::model::project::ProjectJson;
use crate::model::test_case_pass::{TestCasePass, TestCasePassJson, TestCasePassNew};
use crate::plumbing::project::add_project;
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn get_upload(
    pool: web::Data<Pool>,
    item: &xunit_repo_interface::Upload,
) -> Result<crate::model::project::Project, diesel::result::Error> {
    println!("got:{:#?}", item);
    add_project(
        pool.clone(),
        item.project.sk.as_ref(),
        item.project.identiifier.as_ref(),
        item.project.human_name.as_ref(),
    )
}
