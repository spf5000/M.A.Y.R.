use crate::dao::coffee_store::CoffeeStoreDao;
use actix_web::{post, web, HttpResponse, Responder};
use rust_server_model::coffee_store::{GetCoffeeStoreRequest, GetCoffeeStoreResponse};
use std::sync::Arc;
use crate::error::ServerError;

#[post("/coffee/get")]
pub async fn get_coffee_store(
    request: web::Bytes,
    handler: web::Data<GetCoffeeStoreHandler>,
) -> impl Responder {
    let request = match serde_json::from_slice(&request) {
        Result::Ok(request) => request,
        Result::Err(err) => {
            // TODO: Invalid input request?
            log::error!("Failed to deserialize request: {:?}", request);
            return ServerError::from(err).into()
        }
    };
    match handler.handle(request) {
        Ok(response) => response,
        Err(err) => {
            log::error!("Error returned by the handler: {}", err);
            err.into()
        }
    }
}

#[derive(Clone)]
pub struct GetCoffeeStoreHandler {
    coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
}

impl GetCoffeeStoreHandler {
    pub fn new(
        coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
    ) -> GetCoffeeStoreHandler {
        GetCoffeeStoreHandler { coffee_store_dao }
    }

    fn handle(&self, request: GetCoffeeStoreRequest) -> Result<HttpResponse, ServerError> {
        let response = GetCoffeeStoreResponse {
            coffee_store_details: self.coffee_store_dao.get_store_by_id(&request.coffee_store_id)?
        };
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&response)?))
    }
}
