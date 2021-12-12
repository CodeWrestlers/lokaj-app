pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct Db {
    pool: PgPool,
}

impl Db {
    pub fn get_connection(&self) -> Result<PgPooledConnection, PoolError> {
        self.pool.get()
    }
}

// TODO: pool size parameter as environment var
lazy_static! {
    pub static ref DB: Db = {
        let cm = ConnectionManager::<PgConnection>::new(get_connection_string());
        Db {
            pool: Pool::builder().max_size(4).build(cm).unwrap(),
        }
    };
}

fn get_connection_string() -> String {
    dotenv().ok();

    let database_url_env = env::var("DATABASE_URL");

    match database_url_env {
        Ok(url) => url,
        Err(_) => {
            let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
            let database_pass = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
            let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
            format!(
                "postgres://{}:{}@postgres/{}",
                database_user, database_pass, database_name
            )
        }
    }
}
