/**
** Application for storing and updating information
** about all applications of the system
**/

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate log4rs;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod database;
mod routes;
mod models;
mod schema;

use actix_redis::RedisSession;
use actix_service::Service;
use actix_web::{App, HttpServer};

lazy_static! {
    static ref SETTINGS: models::configuration::AppSettings = {
        use config::{Config, File as ConfigFile};

        let mut config_data = Config::new();
        config_data
            .merge(ConfigFile::with_name("configurations/config.toml"))
            .unwrap();
        config_data
            .try_into()
            .expect("Error initializing application settings from the config.toml file; crashing!")
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("configurations/log4rs.yaml", Default::default())
        .unwrap();

    warn!("Run main server...");

    let app_server = move || {
        App::new()
            .wrap_fn(|req, srv| {
                info!("REQUEST: {:?}", req.head());
                srv.call(req)
            })
            .wrap(RedisSession::new(
                format!("{}:{}", &SETTINGS.redis.host, SETTINGS.redis.port),
                &[0; 32],
            ))
            .wrap(routes::error::error_handlers())
            .configure(routes::config_server)
    };

    warn!("Starting Server!");
    HttpServer::new(app_server)
        .bind(format!(
            "{}:{}",
            &SETTINGS.server.host, SETTINGS.server.port,
        ))?
        .run()
        .await
}
