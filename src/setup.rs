use crate::config::{Config, DatabaseConfig};
use crate::log;
use crate::routes::healthcheck::health_check;
use crate::routes::index::index_page;
use crate::routes::user::{activate_user, register};
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{dev::Server, HttpServer};
use actix_web::{web, App};
use secrecy::Secret;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Container for the Actix Application
pub struct Application {
    pub port: u16,
    actix_server: Server,
}

impl Application {
    /// Create a actix application from a [`Config`]
    ///
    /// Takes in a [`Config`] and creates a actix web server according to it.
    /// Returns an error if there are network related issues.
    pub async fn with_config(config: Config) -> Result<Self, anyhow::Error> {
        let db_pool = get_database_pool(config.database);
        let address = format!("127.0.0.1:{}", config.app.port);
        let sengrid_key = config.app.sendgrid_key;
        let base_url = config.app.base_url;
        let socket = TcpListener::bind(address)?;
        let port = socket.local_addr()?.port();
        let server = run(socket, db_pool, config.app.pepper, sengrid_key, base_url).await?;
        Ok(Self {
            port,
            actix_server: server,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.actix_server.await
    }
}

/// Build a [`PgPool`] from a [`DatabaseConfig`]
fn get_database_pool(config: DatabaseConfig) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.as_connect_options())
}

#[derive(Debug)]
pub struct AppBaseUrl(pub String);

pub struct SendGridApiKey(pub Secret<String>);

pub struct Pepper(pub Secret<String>);

/// Construct the actix server instance based on the passed parameters.
///
/// The actix server gets built here, supplying every information necessary (routes, app data, etc.).
/// Throws an error if there are issues construction the actix server.
///
/// * `socket` - [`TcpListener`] to bind the server to
/// * `db_pool` - [`PgPool`] to use for data storage
/// * `pepper` - String that gets added to Password Hashes. For further information see [Pepper](https://en.wikipedia.org/wiki/Pepper_(cryptography))
async fn run(
    socket: TcpListener,
    db_pool: PgPool,
    pepper: Secret<String>,
    sendgrid_key: Secret<String>,
    base_url: String,
) -> Result<Server, anyhow::Error> {
    log::configure_subscriber();

    let db_pool = Data::new(db_pool);
    let pepper = Data::new(Pepper(pepper));
    let sendgrid = Data::new(SendGridApiKey(sendgrid_key));
    let base = Data::new(AppBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(index_page))
            .route("/healthcheck", web::get().to(health_check))
            .route("/register", web::get().to(register))
            .service(activate_user)
            .service(Files::new("/static", "./static").prefer_utf8(true))
            .app_data(db_pool.clone())
            .app_data(pepper.clone())
            .app_data(sendgrid.clone())
            .app_data(base.clone())
    })
    .listen(socket)?
    .run();
    Ok(server)
}
