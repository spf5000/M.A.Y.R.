use rust_server_model::coffee_store::{CoffeeStoreManifest, CoffeeStoreSummary, CoffeeStoreDetails};
use std::vec::Vec;
use crate::error::ServerError;

pub trait CoffeeStoreDao {
    fn list_stores(self: &Self) -> Result<Vec<CoffeeStoreSummary>, ServerError>;
    fn create_store(self: &Self, coffee_store: CoffeeStoreManifest) -> Result<CoffeeStoreDetails, ServerError>;
}