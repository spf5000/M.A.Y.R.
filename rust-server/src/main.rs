// use crate::dao::coffee_store::HashMapCoffeeStoreDao ;
use crate::dao::coffee_store::MongoCoffeeStoreDao;
use crate::handlers::{ListCoffeeStoresHandler, GetCoffeeStoreHandler, CreateCoffeeStoreHandler};
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use std::sync::Arc;
use mongodb::sync::Client;
use mongodb::options::{ClientOptions, StreamAddress};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::env;

mod dao;
mod handlers;
mod error;

const DATABASE_HOSTNAME_ENV_NAME: &str = "DATABASE_HOSTNAME";
const DATABASE_PORT_ENV_NAME: &str = "DATABASE_PORT";
const DATABASE_HOSTNAME_FALLBACK: &str = "localhost";
const DATABASE_PORT_FALLBACK: u16 = 27017;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hostname = env::var(DATABASE_HOSTNAME_ENV_NAME).unwrap_or(String::from(DATABASE_HOSTNAME_FALLBACK));
    let port = env::var(DATABASE_PORT_ENV_NAME)
        .map(|port_string| port_string.parse::<u16>().unwrap())
        .unwrap_or(DATABASE_PORT_FALLBACK);

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let client = Client::with_options(ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname,
            port: Some(port)
        }])
        .build()).map_err(|err| {
        println!("Client Error: {:?}", err);
        IoError::from(IoErrorKind::ConnectionRefused)
    })?;

    let coffee_store_dao = Arc::new(MongoCoffeeStoreDao::new(client));

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .data(ListCoffeeStoresHandler::new(coffee_store_dao.clone()))
            .data(CreateCoffeeStoreHandler::new(coffee_store_dao.clone()))
            .data(GetCoffeeStoreHandler::new(coffee_store_dao.clone()))
            .wrap(logger)
            // TODO: Figure out CORS being safe
            .wrap(Cors::permissive())
            .service(handlers::list_coffee_stores)
            .service(handlers::create_coffee_store)
            .service(handlers::get_coffee_store)
    })
    .bind("0.0.0.0:9080")?
    .run()
    .await
}
