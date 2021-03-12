use super::CoffeeStoreDao;
use rust_server_model::coffee_model::CoffeeStore;
use std::collections::HashMap;
use std::vec::Vec;
use uuid::Uuid;
use std::sync::RwLock;

// #[derive(Clone)]
pub struct HashMapCoffeeStoreDao {
    store_map: RwLock<HashMap<String, CoffeeStore>>
}

impl HashMapCoffeeStoreDao {
    pub fn new() -> HashMapCoffeeStoreDao {
        let mut store_map = HashMap::new();
        store_map.insert(
            "test".to_string(),
            CoffeeStore {
                name: "Rosolini's".to_string(),
                avg_rating: Option::None,
            },
        );

        HashMapCoffeeStoreDao {
            store_map: RwLock::new(store_map)
        }
    }
}

impl CoffeeStoreDao for HashMapCoffeeStoreDao {
    fn list_stores(&self) -> Result<Vec<CoffeeStore>, String> {
        let store_map = self.store_map.read().unwrap();
        Ok(store_map.values().cloned().collect())
    }

    fn create_store(&self, coffee_store: CoffeeStore) -> Result<(), String>{
        let id: String = Uuid::new_v4().to_string();
        println!("Inserting {:?} with id {}", coffee_store, id);
        let mut store_map = self.store_map.write().unwrap();
        store_map.insert(id, coffee_store);

        Ok(())
    }
}
