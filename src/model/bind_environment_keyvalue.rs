use crate::model::environment;
use crate::model::keyvalue;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[table_name = "bind_environment_keyvalue"]
#[belongs_to(environment::Environment, foreign_key = "fk_environment")]
#[belongs_to(keyvalue::KeyValue, foreign_key = "fk_keyvalue")]
pub struct BindEnvironmentKeyvalue {
    pub id: i32,
    pub fk_environment: i32,
    pub fk_keyvalue: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "bind_environment_keyvalue"]
pub struct BindEnvironmentKeyvalueNew {
    pub fk_environment: i32,
    pub fk_keyvalue: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindEnvironmentKeyvalueJson {
    pub fk_environment: i32,
    pub fk_keyvalue: i32,
}
