use crate::SETTINGS;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod create;
pub mod read;
pub mod update;
pub mod delete;

lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = {
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            &SETTINGS.db.user,
            &SETTINGS.db.pass,
            &SETTINGS.db.host,
            SETTINGS.db.port,
            &SETTINGS.db.db_name
        );
        debug!("db url: {}", &db_url);
        let manager = ConnectionManager::<PgConnection>::new(db_url);

        Pool::builder()
            .max_size(SETTINGS.db.pool_size)
            .build(manager)
            .expect("Failed to initialize the DB connection pool")
    };
}