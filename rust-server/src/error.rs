use std::error::Error;
use std::fmt::{Formatter, Result, Display};
use std::boxed::Box;
use std::convert::From;
use std::borrow::Borrow;
use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};
use actix_web::web::HttpResponse;

#[derive(Debug)]
pub enum ServerErrorType {
    Unknown(String),
    NotFound(String)
}

#[derive(Debug)]
pub struct ServerError {
    pub error_type: ServerErrorType,
    pub source: Option<Box<dyn Error>>,
}

impl ServerError {
    pub fn not_found(reason: String) -> Self {
        ServerError {
            error_type: ServerErrorType::NotFound(reason),
            source: None
        }
    }
}

impl Into<HttpResponse> for ServerError {
    fn into(self) -> HttpResponse {
        match self.error_type {
            ServerErrorType::Unknown(_reason) => HttpResponse::InternalServerError().body("Internal error"),
            ServerErrorType::NotFound(reason) => HttpResponse::NotFound().body(reason)
        }
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.error_type {
            ServerErrorType::Unknown(reason) => write!(f, "{}", reason),
            ServerErrorType::NotFound(reason) => write!(f, "{}", reason),
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

impl From<serde_json::Error> for ServerError {
    fn from(error: serde_json::Error) -> Self {
        ServerError {
            error_type: ServerErrorType::Unknown(error.to_string()),
            source: Some(Box::new(error))
        }
    }
}

impl From<mongodb::bson::de::Error> for ServerError {
    fn from(error: mongodb::bson::de::Error) -> Self {
        ServerError {
            error_type: ServerErrorType::Unknown(error.to_string()),
            source: Some(Box::new(error))
        }
    }
}

impl From<mongodb::bson::ser::Error> for ServerError {
    fn from(error: mongodb::bson::ser::Error) -> Self {
        ServerError {
            error_type: ServerErrorType::Unknown(error.to_string()),
            source: Some(Box::new(error))
        }
    }
}

impl From<mongodb::error::Error> for ServerError {
    fn from(error: mongodb::error::Error) -> Self {
        ServerError {
            error_type: ServerErrorType::Unknown(error.to_string()),
            source: Some(Box::new(error))
        }
    }
}

// Used by HashMapCoffeeStoreDao
impl <T> From<PoisonError<RwLockReadGuard<'_, T>>> for ServerError {
    fn from(error: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        ServerError {
            error_type: ServerErrorType::Unknown(error.to_string()),
            source: None
        }
    }
}

impl <T> From<PoisonError<RwLockWriteGuard<'_, T>>> for ServerError {
    fn from(error: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        ServerError {
            error_type: ServerErrorType::Unknown(error.to_string()),
            source: None
        }
    }
}