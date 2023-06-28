use crate::schema::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Environment {
    pub id: i32,
    pub sk: String,
    pub hash_keyvalue: String,
    pub best_before: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = environment)]
pub struct EnvironmentNew<'a> {
    pub sk: &'a str,
    pub hash_keyvalue: &'a str,
    pub best_before: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentJson {
    pub sk: Option<String>,
    pub key_value: Option<HashMap<String, String>>,
}
