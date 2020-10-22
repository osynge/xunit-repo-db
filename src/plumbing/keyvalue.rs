use crate::model::keyvalue::{KeyValue, KeyValueJson, KeyValueNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_keyvalue(
    pool: web::Data<Pool>,
    item: &KeyValueJson,
) -> Result<KeyValue, diesel::result::Error> {
    use crate::schema::keyvalue::dsl::*;
    let db_connection = pool.get().unwrap();
    match keyvalue
        .filter(key.eq(&item.key))
        .filter(value.eq(&item.value))
        .first::<KeyValue>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = KeyValueNew {
                key: &item.key,
                value: &item.value,
            };

            insert_into(keyvalue)
                .values(&new_keyvalue)
                .execute(&db_connection)
                .expect("Error saving new keyvalue");

            let result = keyvalue.order(id.desc()).first(&db_connection).unwrap();
            Ok(result)
        }
    }
}
