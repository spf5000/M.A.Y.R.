use rust_server_model::coffee_model::CoffeeStore;
use std::vec::Vec;

pub trait CoffeeStoreDao {
    fn list_stores(self: &Self) -> Result<Vec<CoffeeStore>, String>;
    fn create_store(self: &Self, coffee_store: CoffeeStore) -> Result<(), String>;
}