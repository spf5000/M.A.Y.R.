use crate::dao::coffee_store::CoffeeStoreDao;
use actix_web::{post, web, HttpResponse, Responder};
use rust_server_model::coffee_store::{CreateCoffeeStoreRequest, CreateCoffeeStoreResponse};
use std::sync::Arc;
use crate::error::ServerError;

#[post("/coffee/create")]
pub async fn create_coffee_store(
    request: web::Bytes,
    handler: web::Data<CreateCoffeeStoreHandler>,
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
pub struct CreateCoffeeStoreHandler {
    coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
}

impl CreateCoffeeStoreHandler {
    pub fn new(
        coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
    ) -> CreateCoffeeStoreHandler {
        CreateCoffeeStoreHandler { coffee_store_dao }
    }

    // fn handle(&self, request: CreateCoffeeStoreRequest) -> HttpResponse {
    //     match self.coffee_store_dao.create_store(request.coffee_store) {
    //         Ok(coffee_store_details) => {
    //             let response = CreateCoffeeStoreResponse {
    //                 coffee_store_details
    //             };
    //             match serde_json::to_string(&response) {
    //                 Ok(body) => HttpResponse::Ok()
    //                     .content_type("application/json")
    //                     .body(body),
    //                 Err(err) => {
    //                     log::error!("Failed to serialize CreateCoffeeStore response: {}", err);
    //                     create_internal_error()
    //                 }
    //             }
    //         },
    //         Err(err) => {
    //             log::error!("DAO error when creating store: {}", err);
    //             create_internal_error()
    //         }
    //     }
    // }

    fn handle(&self, request: CreateCoffeeStoreRequest) -> Result<HttpResponse, ServerError> {
        let response = CreateCoffeeStoreResponse {
            coffee_store_details: self.coffee_store_dao.create_store(request.coffee_store)?
        };
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&response)?))
    }
}