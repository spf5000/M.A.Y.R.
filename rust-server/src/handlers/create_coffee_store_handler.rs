use crate::dao::coffee_store::CoffeeStoreDao;
use actix_web::{post, web, HttpResponse, Responder};
use log::info;
use rust_server_model::coffee_model::CoffeeStore;
use std::sync::Arc;

#[post("/coffee/create")]
pub async fn create_coffee_store(
    request: web::Bytes,
    handler: web::Data<CreateCoffeeStoreHandler>,
) -> impl Responder {
    info!("Bytes {:?}", request);
    let coffee_store = match serde_json::from_slice(&request) {
        Result::Ok(coffee_store) => coffee_store,
        Result::Err(err) => panic!(err),
    };
    handler.handle(coffee_store)
}

#[derive(Clone)]
pub struct CreateCoffeeStoreHandler {
    coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
}

impl CreateCoffeeStoreHandler {
    pub fn new(
        coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
    ) -> CreateCoffeeStoreHandler {
        CreateCoffeeStoreHandler { coffee_store_dao }
    }

    fn handle(&self, coffee_store: CoffeeStore) -> HttpResponse {
        match self.coffee_store_dao.create_store(coffee_store) {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::InternalServerError().body(err)
        }
    }
}