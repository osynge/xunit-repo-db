use crate::ConnectionError;
use crate::Pool;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn establish_connection_pool(db_url: &str, create_db: bool) -> Result<Pool, ConnectionError> {
    if create_db {
        let mut connection = PgConnection::establish(db_url).unwrap();
        run_migration(&mut connection);
    };
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let database_pool = Pool::new(manager).expect("Postgres connection pool could not be created");
    run_migration(&mut database_pool.get().unwrap());
    Ok(database_pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::project::ProjectNew;
    use diesel::dsl::insert_into;
    use std::sync::Mutex;
    use uuid;

    static TEST_MUTEX: Mutex<i32> = Mutex::new(1);

    #[derive(Deserialize, Debug)]
    struct DbUrl {
        password: String,
        username: String,
        hostname: String,
        #[serde(default = "db_url_default_port")]
        port: u16,
        database: String,
    }

    impl DbUrl {
        fn as_url(&self) -> String {
            ///postgres://postgres:newpassword@localhost/diesel_demo///
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.hostname, self.port, self.database,
            )
        }
    }

    fn db_url_default_port() -> u16 {
        5432
    }

    fn db_url_get() -> String {
        let prefix = "DB_URL_";
        let dburl = match envy::prefixed(prefix).from_env::<DbUrl>() {
            Ok(p) => p,
            Err(err) => match (err) {
                envy::Error::MissingValue(p) => {
                    panic!(
                        "Missing environment variable: {}{}",
                        prefix,
                        p.to_uppercase()
                    )
                }
                envy::Error::Custom(p) => {
                    panic!("{}", p);
                }
            },
        };
        println!("{:?}", dburl.as_url());
        println!("{:?}", dburl);
        dburl.as_url()
    }

    fn get_uuid_as_string() -> String {
        let uuid_human_name = uuid::Uuid::new_v4();
        let mut long_string = uuid_human_name.to_string();
        long_string.truncate(32);
        long_string
    }

    #[test]
    fn establish_connection_in_mem() {
        use crate::schema::project::dsl::*;
        let _guard = TEST_MUTEX.lock().unwrap();
        let mut conn = establish_connection_pool(db_url_get().as_str(), true)
            .unwrap()
            .get()
            .unwrap();
        conn.begin_test_transaction();
        let str_sk = get_uuid_as_string();
        let str_identifier = get_uuid_as_string();
        let uuid_human_name = get_uuid_as_string();
        let new_link = ProjectNew {
            sk: &str_sk,
            identifier: &str_identifier,
            human_name: &uuid_human_name,
        };
        let flink = insert_into(project)
            .values(&new_link)
            .execute(&mut conn)
            .expect("Error saving new post");
    }
    #[test]
    fn establish_connection_2() {
        use crate::schema::project::dsl::*;
        let _guard = TEST_MUTEX.lock().unwrap();
        //let conn = establish_connection(db_url_get().as_str());
        //run_migrations(&conn);
        let mut conn = establish_connection_pool(db_url_get().as_str(), true)
            .unwrap()
            .get()
            .unwrap();

        let str_sk = get_uuid_as_string();
        let str_identifier = get_uuid_as_string();
        let uuid_human_name = get_uuid_as_string();

        conn.begin_test_transaction();
        let new_link2 = ProjectNew {
            sk: &str_sk,
            identifier: &str_identifier,
            human_name: &uuid_human_name,
        };
        let _ = insert_into(project)
            .values(&new_link2)
            .execute(&mut conn)
            .expect("Error saving new post");
    }
}
