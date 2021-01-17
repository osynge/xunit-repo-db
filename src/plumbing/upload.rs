use crate::model::project::ProjectJson;
use crate::model::test_case_pass::{TestCasePass, TestCasePassJson, TestCasePassNew};
use crate::plumbing::enviroment::add_enviroment;
use crate::plumbing::project::add_project;
use crate::plumbing::run_identifier::add_run_identifier;
use crate::plumbing::test_case::add_test_case;
use crate::plumbing::test_case_error::add_test_case_error;
use crate::plumbing::test_case_pass::add_test_case_pass;
use crate::plumbing::test_file::add_test_file;
use crate::plumbing::test_file_run::add_test_file_run;
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use super::{test_case, test_file};

pub fn get_upload(
    pool: web::Data<Pool>,
    item: &xunit_repo_interface::Upload,
) -> Result<crate::model::project::Project, diesel::result::Error> {
    let conn = pool.clone().get().unwrap();
    println!("got:{:#?}", item);
    let project = add_project(
        &conn,
        item.project.sk.as_ref(),
        item.project.identiifier.as_ref(),
        item.project.human_name.as_ref(),
    )?;
    println!("project:{:#?}", project);
    let env = add_enviroment(
        &conn,
        project.id,
        item.enviroment.sk.as_ref(),
        Some(&item.enviroment.key_value),
    )?;
    println!("env:{:#?}", env);
    let run = add_run_identifier(
        &conn,
        project.id,
        item.run.sk.as_ref(),
        item.run.client_identifier.as_ref(),
        None,
    )?;
    println!("run:{:#?}", run);
    for fileItem in item.files.iter() {
        let dir = &fileItem.directory;
        let name = &fileItem.filename;
        let test_file = add_test_file(&conn, dir, name)?;
        let test_file_run = add_test_file_run(&conn, test_file.id, run.id)?;

        for ts in fileItem.content.testsuite.iter() {
            for tc in ts.testcase.iter() {
                let test_case = add_test_case(&conn, &tc.name, &tc.classname)?;
                match (&tc.skipped, &tc.failure, &tc.error) {
                    (Some(skipmsg), None, None) => {
                        println!("Skip");
                    }
                    (None, Some(failmsg), None) => {
                        println!("fail");
                    }
                    (None, None, Some(tc_error)) => {
                        add_test_case_error(
                            pool.clone(),
                            test_file_run.id,
                            test_case.id,
                            Some(tc.time),
                            Some(&tc_error.message.clone()),
                            Some(&tc_error.error_type.clone()),
                            Some(&tc_error.description.clone()),
                            tc.system_out.as_ref(),
                            tc.system_err.as_ref(),
                        )?;
                    }
                    (None, None, None) => {
                        println!("Pass");
                        add_test_case_pass(pool.clone(), run.id, test_case.id, &Some(tc.time))?;
                    }
                    _ => {
                        println!("Cannot mix");
                    }
                }
            }
        }
    }
    Ok(project)
}
