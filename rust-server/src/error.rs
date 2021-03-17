use std::error::Error;
use std::fmt::{Formatter, Result, Display};
use std::boxed::Box;
use std::convert::From;
use std::borrow::Borrow;
use actix_web::web::HttpResponse;

#[derive(Debug)]
pub enum ServerErrorType {
    UNKNOWN(String)
}

#[derive(Debug)]
pub struct ServerError {
    error_type: ServerErrorType,
    source: Option<Box<dyn Error>>,
}

impl Into<HttpResponse> for ServerError {
    fn into(self) -> HttpResponse {
        match self.error_type {
            ServerErrorType::UNKNOWN(_reason) => HttpResponse::InternalServerError().body("Internal error")
        }
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.error_type {
            ServerErrorType::UNKNOWN(reason) => write!(f, "{}", reason)
        }
    }
}

impl Error for ServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(reference) = &self.source {
            Some(reference.borrow())
        } else {
            None
        }
    }
}

// impl<T> From<T> for ServerError where T: Error {
//     fn from(error: T) -> Self {
//         ServerError {
//             error_type: ServerErrorType::UNKNOWN(error.to_string()),
//             source: Some(Box::new(error))
//         }
//     }
// }
//
impl From<serde_json::Error> for ServerError {
    fn from(error: serde_json::Error) -> Self {
        ServerError {
            error_type: ServerErrorType::UNKNOWN(error.to_string()),
            source: Some(Box::new(error))
        }
    }
}

impl From<mongodb::bson::de::Error> for ServerError {
    fn from(error: mongodb::bson::de::Error) -> Self {
        ServerError {
            error_type: ServerErrorType::UNKNOWN(error.to_string()),
            source: Some(Box::new(error))
        }
    }
}

impl From<mongodb::bson::ser::Error> for ServerError {
    fn from(error: mongodb::bson::ser::Error) -> Self {
        ServerError {
            error_type: ServerErrorType::UNKNOWN(error.to_string()),
            source: Some(Box::new(error))
        }
    }
}

impl From<mongodb::error::Error> for ServerError {
    fn from(error: mongodb::error::Error) -> Self {
        ServerError {
            error_type: ServerErrorType::UNKNOWN(error.to_string()),
            source: Some(Box::new(error))
        }
    }
}
