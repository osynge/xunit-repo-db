use crate::model::enviroment;
use crate::model::run_identifier;
use crate::schema::*;
use serde::{Deserialize, Serialize};

/*

CREATE TABLE test_run (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) NOT NULL,
    created BigInt NOT NULL,
    fk_run_identifier INTEGER NOT NULL,
    fk_enviroment INTEGER NOT NULL,
    FOREIGN KEY (fk_enviroment) REFERENCES enviroment (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_run_identifier) REFERENCES run_identifier (id) ON DELETE CASCADE ON UPDATE NO ACTION
); */

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "test_run"]
#[belongs_to(run_identifier::RunIdentifier, foreign_key = "fk_run_identifier")]
#[belongs_to(enviroment::Enviroment, foreign_key = "fk_enviroment")]
pub struct TestRun {
    pub id: i32,
    pub sk: String,
    pub created: i32,
    pub fk_run_identifier: i32,
    pub fk_enviroment: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "test_run"]
pub struct TestRunNew<'a> {
    pub sk: &'a str,
    pub created: i64,
    pub fk_run_identifier: i32,
    pub fk_enviroment: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestRunJson {
    pub sk: String,
    pub created: i32,
    pub fk_run_identifier: i32,
    pub fk_enviroment: i32,
}
