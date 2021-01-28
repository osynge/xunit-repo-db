use crate::model::enviroment::EnviromentJson;
use crate::model::keyvalue::KeyValueJson;
use crate::model::project::ProjectJson;
use crate::model::run_identifier::RunIdentifierJson;
use crate::model::test_case_error::TestCaseErrorJson;
use crate::model::test_case_failure::TestCaseFailureJson;
use crate::model::test_case_pass::TestCasePassJson;
use crate::model::test_case_skipped::TestCaseSkippedJson;
use crate::Pool;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;

pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../../templates/index.html")))
}
