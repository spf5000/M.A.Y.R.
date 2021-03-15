use crate::dao::coffee_store::CoffeeStoreDao;
use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Arc;
use rust_server_model::coffee_store::{ListCoffeeStoresRequest, ListCofeeStoresResponse};
use crate::utils::error_utils::create_internal_error;

#[post("/coffee/list")]
pub async fn list_coffee_stores(
    request: web::Bytes,
    handler: web::Data<ListCoffeeStoresHandler>,
) -> impl Responder {
    log::info!("Listing Coffee Stores");
    let request = match serde_json::from_slice(&request) {
        Result::Ok(request) => request,
        Result::Err(err) => {
            // TODO: Invalid input request?
            log::error!("Failed to deserialize request: {:?}", request);
            return create_internal_error()
        }
    };
    handler.handle(request)
}

#[derive(Clone)]
pub struct ListCoffeeStoresHandler {
    coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
}

impl ListCoffeeStoresHandler {
    pub fn new(
        coffee_store_dao: Arc<dyn CoffeeStoreDao + Send + Sync>,
    ) -> ListCoffeeStoresHandler {
        ListCoffeeStoresHandler { coffee_store_dao }
    }

    fn handle(&self, _req: ListCoffeeStoresRequest) -> HttpResponse {
        // TODO: paginate DAO
        let response = self.coffee_store_dao.list_stores();
        match response {
            Ok(stores) => {
                let response = ListCofeeStoresResponse {
                    coffee_stores: stores,
                    next_token: Option::None
                };
                match serde_json::to_string(&response) {
                    Ok(body) => HttpResponse::Ok()
                        .content_type("application/json")
                        .body(body),
                    Err(err) => {
                        log::error!("Failed to serialize CreateCoffeeStore response: {}", err);
                        create_internal_error()
                    }
                }
            },
            Err(err) => {
                create_internal_error()
            }
        }
    }
}
