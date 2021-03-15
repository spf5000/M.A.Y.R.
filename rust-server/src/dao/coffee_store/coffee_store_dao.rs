use rust_server_model::coffee_store::{CoffeeStoreManifest, CoffeeStoreSummary, CoffeeStoreDetails};
use std::vec::Vec;

pub trait CoffeeStoreDao {
    fn list_stores(self: &Self) -> Result<Vec<CoffeeStoreSummary>, String>;
    fn create_store(self: &Self, coffee_store: CoffeeStoreManifest) -> Result<CoffeeStoreDetails, String>;
}