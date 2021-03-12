use crate::dao::coffee_store::CoffeeStoreDao;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use log::info;
use std::sync::Arc;

#[post("/coffee/list")]
pub async fn list_coffee_stores(
    request: HttpRequest,
    handler: web::Data<ListCoffeeStoresHandler>,
) -> impl Responder {
    info!("Listing Coffee Stores");
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

    fn handle(&self, _req: HttpRequest) -> HttpResponse {
        let response = self.coffee_store_dao.list_stores();
        match response {
            Ok(stores) => {
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&stores).unwrap())
            },
            Err(err) => {
                HttpResponse::InternalServerError().body(err)
            }
        }
    }
}
