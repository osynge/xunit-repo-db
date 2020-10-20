use crate::model::enviroment;
use crate::model::keyvalue;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "bind_enviroment_keyvalue"]
#[belongs_to(enviroment::Enviroment, foreign_key = "fk_enviroment")]
#[belongs_to(keyvalue::KeyValue, foreign_key = "fk_keyvalue")]
pub struct BindEnviromentKeyvalue {
    pub id: i32,
    pub fk_enviroment: i32,
    pub fk_keyvalue: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "bind_enviroment_keyvalue"]
pub struct BindEnviromentKeyvalueNew {
    pub fk_enviroment: i32,
    pub fk_keyvalue: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindEnviromentKeyvalueJson {
    pub fk_enviroment: i32,
    pub fk_keyvalue: i32,
}
