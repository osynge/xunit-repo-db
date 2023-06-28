use crate::model::environment::Environment;
use crate::model::keyvalue::KeyValue;
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[diesel(table_name = bind_environment_keyvalue)]
#[belongs_to(Environment, foreign_key = "fk_environment")]
#[belongs_to(KeyValue, foreign_key = "fk_keyvalue")]
pub struct BindEnvironmentKeyvalue {
    pub id: i32,
    pub fk_environment: i32,
    pub fk_keyvalue: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = bind_environment_keyvalue)]
pub struct BindEnvironmentKeyvalueNew {
    pub fk_environment: i32,
    pub fk_keyvalue: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindEnvironmentKeyvalueJson {
    pub fk_environment: i32,
    pub fk_keyvalue: i32,
}
