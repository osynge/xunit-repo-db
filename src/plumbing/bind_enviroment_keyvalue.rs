use crate::model::bind_enviroment_keyvalue::{
    BindEnviromentKeyvalue, BindEnviromentKeyvalueJson, BindEnviromentKeyvalueNew,
};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_bind_enviroment_keyvalue(
    pool: web::Data<Pool>,
    item: &BindEnviromentKeyvalueJson,
) -> Result<BindEnviromentKeyvalue, diesel::result::Error> {
    use crate::schema::bind_enviroment_keyvalue::dsl::*;
    let db_connection = pool.get().unwrap();
    match bind_enviroment_keyvalue
        .filter(fk_enviroment.eq(&item.fk_enviroment))
        .filter(fk_keyvalue.eq(&item.fk_keyvalue))
        .first::<BindEnviromentKeyvalue>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = BindEnviromentKeyvalueNew {
                fk_enviroment: item.fk_enviroment,
                fk_keyvalue: item.fk_keyvalue,
            };

            insert_into(bind_enviroment_keyvalue)
                .values(&new_keyvalue)
                .execute(&db_connection)
                .expect("Error saving new keyvalue");

            let result = bind_enviroment_keyvalue
                .order(id.desc())
                .first(&db_connection)
                .unwrap();
            Ok(result)
        }
    }
}
