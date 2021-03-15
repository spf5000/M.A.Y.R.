use actix_web::{HttpResponse};

pub fn create_internal_error() -> HttpResponse {
    HttpResponse::InternalServerError().body("Internal error")
}