use std::collections::HashMap;

use crate::model::bind_enviroment_keyvalue::BindEnviromentKeyvalueJson;
use crate::model::enviroment::{Enviroment, EnviromentJson, EnviromentNew};
use crate::model::keyvalue::KeyValueJson;
use crate::plumbing::bind_enviroment_keyvalue::add_bind_enviroment_keyvalue;
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

fn keyvalue_hash_gen(keyvalue: &HashMap<String, String>) -> String {
    let mut sorted: Vec<_> = keyvalue.iter().collect();
    sorted.sort();
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    sorted.hash(&mut hasher);
    hasher.finish().to_string()
}

fn insert_enviroment(
    pool: web::Data<Pool>,
    fk_project: i32,
    sk: &String,
    keyvalue: &HashMap<String, String>,
    keyvalue_hash: &String,
) -> Result<Enviroment, diesel::result::Error> {
    let keyvalue_hash = keyvalue_hash_gen(keyvalue);

    let insert = enviroment_insert_sk_hash_keyvalue(pool.clone(), fk_project, &sk, &keyvalue_hash);
    match insert {
        Ok(env) => {
            for (key, value) in keyvalue.into_iter() {
                let added_kv = match add_keyvalue(pool.clone(), key, value) {
                    Ok(p) => p.id,
                    Err(p) => {
                        return Err(p);
                    }
                };
                let new = BindEnviromentKeyvalueJson {
                    fk_enviroment: env.id,
                    fk_keyvalue: added_kv,
                };
                match add_bind_enviroment_keyvalue(pool.clone(), &new) {
                    Ok(_) => {}
                    Err(anerr) => return Err(anerr),
                }
            }
            Ok(env)
        }
        Err(p) => Err(p),
    }
}

pub fn add_enviroment(
    pool: web::Data<Pool>,
    fk_project: i32,
    enviroment_sk: Option<&String>,
    enviroment_key_value: Option<&HashMap<String, String>>,
) -> Result<Enviroment, diesel::result::Error> {
    match (enviroment_sk, enviroment_key_value) {
        (Some(sk), Some(kv)) => {
            let keyvalue_hash = keyvalue_hash_gen(kv);
            match enviroment_get_by_sk_hash_keyvalue(pool.clone(), fk_project, &sk, &keyvalue_hash)
            {
                Ok(p) => Ok(p),
                Err(_) => insert_enviroment(pool.clone(), fk_project, sk, kv, &keyvalue_hash),
            }
        }

        (None, Some(kv)) => {
            let sk = Uuid::new_v4().to_string();
            let keyvalue_hash = keyvalue_hash_gen(kv);
            match enviroment_get_by_hash_keyvalue(pool.clone(), fk_project, &keyvalue_hash) {
                Ok(p) => Ok(p),
                Err(_) => insert_enviroment(pool.clone(), fk_project, &sk, kv, &keyvalue_hash),
            }
        }
        (None, None) => Err(diesel::result::Error::NotFound),
        (Some(sk), None) => enviroment_get_by_sk(pool.clone(), fk_project, &sk),
    }
}
