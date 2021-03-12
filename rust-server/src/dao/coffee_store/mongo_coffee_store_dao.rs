use super::CoffeeStoreDao;
// use std::sync::Arc;
use mongodb::sync::Client;
use mongodb::bson::{from_document, to_document, Document};
use rust_server_model::coffee_model::CoffeeStore;
use std::result::Result::Err;
use mongodb::sync::Collection;

const DATABASE: &'static str = "test";
const COLLECTION: &'static str = "CoffeeStores";

pub struct MongoCoffeeStoreDao {
    mongo_client: Client
}

impl MongoCoffeeStoreDao {
    // fn new(client: Arc<Client>) -> MongoCoffeeStoreDao {
    pub fn new(client: Client) -> MongoCoffeeStoreDao {
        MongoCoffeeStoreDao {
            mongo_client: client
        }
    }

    fn get_coffee_store_collection(self: &Self) -> Collection {
        let db = self.mongo_client.database(DATABASE);
        db.collection(COLLECTION)
    }
}

impl CoffeeStoreDao for MongoCoffeeStoreDao {
    fn list_stores(self: &Self) -> Result<Vec<CoffeeStore>, String> {
        log::info!("Listing the coffee stores from MongoDB");
        let mut output = vec![];
        let collection = self.get_coffee_store_collection();
        let cursor = collection.find(None, None).map_err(|_err| "mongodb cursor error".to_string())?;
        for result in cursor {
            match result {
                Ok(document) => {
                    let store = from_document(document).map_err(|_err| "mongodb document error".to_string())?;
                    output.push(store);
                },
                Err(_err) => return Err("mongodb error".to_string())
            }
        }

        Ok(output)
    }

    fn create_store(self: &Self, coffee_store: CoffeeStore) -> Result<(), String>{
        log::info!("Creating coffee store {:?} and storing in MongoDB", coffee_store);
        let collection = self.get_coffee_store_collection();
        collection.insert_one(coffee_store_to_document(&coffee_store)?, None)
            .map(|_insert_response| ())
            .map_err(|_err| "mongodb error".to_string())
    }
}

fn coffee_store_to_document(coffee_store: &CoffeeStore) -> Result<Document, String> {
    to_document(coffee_store).map_err(|_err| format!("Failed to serialize coffee store {:?}", coffee_store))
}
