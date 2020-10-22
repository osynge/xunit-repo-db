use crate::model::enviroment::{Enviroment, EnviromentJson, EnviromentNew};
use crate::model::keyvalue::KeyValueJson;
use crate::plumbing::keyvalue::add_keyvalue;
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;

fn enviroment_get_by_sk_hash_keyvalue(
    pool: web::Data<Pool>,
    filter_fk_project: i32,
    filter_sk: &String,
    filter_hash_keyvalue: &String,
) -> Result<Enviroment, diesel::result::Error> {
    use crate::schema::enviroment::dsl::*;
    let db_connection = pool.get().unwrap();
    enviroment
        .filter(sk.eq(filter_sk))
        .filter(fk_project.eq(filter_fk_project))
        .filter(hash_keyvalue.eq(filter_hash_keyvalue))
        .first::<Enviroment>(&db_connection)
}

fn enviroment_get_by_hash_keyvalue(
    pool: web::Data<Pool>,
    filter_fk_project: i32,
    filter_hash_keyvalue: &String,
) -> Result<Enviroment, diesel::result::Error> {
    use crate::schema::enviroment::dsl::*;
    let db_connection = pool.get().unwrap();
    enviroment
        .filter(hash_keyvalue.eq(filter_hash_keyvalue))
        .filter(fk_project.eq(filter_fk_project))
        .first::<Enviroment>(&db_connection)
}

fn enviroment_get_by_sk(
    pool: web::Data<Pool>,
    filter_fk_project: i32,
    filter_sk: &String,
) -> Result<Enviroment, diesel::result::Error> {
    use crate::schema::enviroment::dsl::*;
    let db_connection = pool.get().unwrap();
    enviroment
        .filter(sk.eq(filter_sk))
        .filter(fk_project.eq(filter_fk_project))
        .first::<Enviroment>(&db_connection)
}

fn enviroment_insert_sk_hash_keyvalue(
    pool: web::Data<Pool>,
    insert_fk_project: i32,
    insert_sk: &String,
    insert_hash_keyvalue: &String,
) -> Result<Enviroment, diesel::result::Error> {
    use crate::schema::enviroment::dsl::*;
    let db_connection = pool.get().unwrap();

    let new_link = EnviromentNew {
        sk: &insert_sk,
        hash_keyvalue: &insert_hash_keyvalue,
        best_before: None,
        fk_project: insert_fk_project,
    };
    insert_into(enviroment)
        .values(&new_link)
        .execute(&db_connection)
        .expect("Error saving new enviroment");

    let result = enviroment.order(id.desc()).first(&db_connection).unwrap();
    Ok(result)
}

fn enviroment_insert_hash_keyvalue(
    pool: web::Data<Pool>,
    insert_fk_project: i32,
    insert_hash_keyvalue: &String,
) -> Result<Enviroment, diesel::result::Error> {
    use crate::schema::enviroment::dsl::*;
    let db_connection = pool.get().unwrap();
    let insert_sk = Uuid::new_v4().to_string();
    let new_link = EnviromentNew {
        sk: &insert_sk,
        hash_keyvalue: &insert_hash_keyvalue,
        best_before: None,
        fk_project: insert_fk_project,
    };
    insert_into(enviroment)
        .values(&new_link)
        .execute(&db_connection)
        .expect("Error saving new enviroment");

    let result = enviroment.order(id.desc()).first(&db_connection).unwrap();
    Ok(result)
}

fn keyvalue_hash_gen(keyvalue: &Vec<KeyValueJson>) -> String {
    let mut sorted: Vec<KeyValueJson> = keyvalue.to_vec();
    sorted.sort();
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    sorted.hash(&mut hasher);
    hasher.finish().to_string()
}

pub fn add_enviroment(
    pool: web::Data<Pool>,
    fk_project: i32,
    item: &EnviromentJson,
) -> Result<Enviroment, diesel::result::Error> {
    let mut keyvalue_hash = match &item.key_value {
        Some(p) => Some((keyvalue_hash_gen(p), p)),
        None => None,
    };
    match (&item.sk, &keyvalue_hash) {
        (Some(sk), Some((hash_keyvalue, kv))) => {
            match enviroment_get_by_sk_hash_keyvalue(pool.clone(), fk_project, &sk, &hash_keyvalue)
            {
                Ok(p) => Ok(p),
                Err(_) => {
                    let insert = enviroment_insert_sk_hash_keyvalue(
                        pool.clone(),
                        fk_project,
                        &sk,
                        &hash_keyvalue,
                    );

                    match insert {
                        Ok(p) => {
                            for ding in kv.into_iter() {
                                match add_keyvalue(pool.clone(), ding) {
                                    Ok(p) => {}
                                    Err(p) => {}
                                }
                            }
                            Ok(p)
                        }
                        Err(p) => Err(p),
                    }
                }
            }
        }

        (None, Some((hash_keyvalue, kv))) => {
            match enviroment_get_by_hash_keyvalue(pool.clone(), fk_project, &hash_keyvalue) {
                Ok(p) => Ok(p),
                Err(_) => enviroment_insert_hash_keyvalue(pool.clone(), fk_project, &hash_keyvalue),
            }
        }
        (None, None) => Err(diesel::result::Error::NotFound),
        (Some(sk), None) => enviroment_get_by_sk(pool.clone(), fk_project, &sk),
    }
}
