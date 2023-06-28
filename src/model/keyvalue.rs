use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct KeyValue {
    pub id: i32,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = keyvalue)]
pub struct KeyValueNew<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct KeyValueJson {
    pub key: String,
    pub value: String,
}
