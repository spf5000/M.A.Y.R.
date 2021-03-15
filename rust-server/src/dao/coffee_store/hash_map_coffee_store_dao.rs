use super::CoffeeStoreDao;
use std::collections::HashMap;
use std::vec::Vec;
use uuid::Uuid;
use std::sync::RwLock;
use rust_server_model::coffee_store::{CoffeeStoreSummary, CoffeeStoreManifest, CoffeeStoreDetails};

pub struct HashMapCoffeeStoreDao {
    store_map: RwLock<HashMap<String, CoffeeStoreDetails>>
}

impl HashMapCoffeeStoreDao {
    pub fn new() -> HashMapCoffeeStoreDao {
        HashMapCoffeeStoreDao {
            store_map: RwLock::new(HashMap::new())
        }
    }
}

impl CoffeeStoreDao for HashMapCoffeeStoreDao {
    fn list_stores(&self) -> Result<Vec<CoffeeStoreSummary>, String> {
        let store_map = self.store_map.read().unwrap();
        Ok(store_map.values().cloned()
            .map(|details| CoffeeStoreSummary {
                id: details.id,
                name: details.name,
                avg_rating: details.avg_rating,
            })
            .collect())
    }

    fn create_store(&self, coffee_store: CoffeeStoreManifest) -> Result<CoffeeStoreDetails, String>{
        let id: String = Uuid::new_v4().to_string();
        let coffee_store_details = CoffeeStoreDetails {
            id: id.clone(),
            name: coffee_store.name,
            description: coffee_store.description,
            avg_rating: None
        };

        log::debug!("Inserting {:?} with id {}", &coffee_store_details, &id);
        let mut store_map = self.store_map.write().unwrap();
        store_map.insert(id, coffee_store_details.clone());

        Ok(coffee_store_details)
    }
}
