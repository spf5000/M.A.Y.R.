// use crate::dao::coffee_store::HashMapCoffeeStoreDao ;
use crate::dao::coffee_store::MongoCoffeeStoreDao;
use crate::handlers::{ListCoffeeStoresHandler, GetCoffeeStoreHandler, CreateCoffeeStoreHandler};
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use std::sync::Arc;
use mongodb::sync::Client;
use mongodb::options::{ClientOptions, StreamAddress};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

mod dao;
mod handlers;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // let coffee_store_dao = Arc::new(HashMapCoffeeStoreDao::new());
    let client = Client::with_options(ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: "localhost".into(),
            port: Some(27017)
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
    .bind("127.0.0.1:9080")?
    .run()
    .await
}
