use crate::schema::*;
use crate::ConnectionError;
use crate::Pool;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use thiserror::Error;
use url::{Host, Url};

embed_migrations!();
pub fn run_migrations(conn: &PgConnection) {
    embedded_migrations::run(conn);
}

pub fn establish_connection_pool(db_url: &str, create_db: bool) -> Result<Pool, ConnectionError> {
    if create_db {
        let connection = PgConnection::establish(&db_url).unwrap();
        run_migrations(&connection);
    };
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let database_pool = Pool::new(manager).expect("Postgres connection pool could not be created");
    run_migrations(&database_pool.get().unwrap());
    Ok(database_pool)
}

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::project::ProjectNew;
    use crate::schema::project;
    use diesel::dsl::insert_into;
    use tempdir;

    #[test]
    fn establish_connection_in_mem() {
        use crate::schema::project::dsl::*;
        let conn = establish_connection_pool(
            "postgres://postgres:newpassword@localhost/diesel_demo",
            true,
        )
        .unwrap()
        .get()
        .unwrap();
        conn.begin_test_transaction();

        let new_link = ProjectNew {
            sk: "&toad",
            identifier: "&frog",
            human_name: "&bat",
        };
        let flink = insert_into(project)
            .values(&new_link)
            .execute(&conn)
            .expect("Error saving new post");
    }
    #[test]
    fn establish_connection_2() {
        use crate::schema::project::dsl::*;
        let conn = establish_connection("postgres://postgres:newpassword@localhost/diesel_demo");

        let new_link2 = ProjectNew {
            sk: "&jelly",
            identifier: "&swig",
            human_name: "&song",
        };
        let flink = insert_into(project)
            .values(&new_link2)
            .execute(&conn)
            .expect("Error saving new post");
    }
}
