use super::CoffeeStoreDao;
use mongodb::sync::Client;
use mongodb::bson::{from_document, to_document};
use rust_server_model::coffee_store::{CoffeeStoreDetails, CoffeeStoreSummary, CoffeeStoreManifest};
use crate::error::ServerError;
use mongodb::sync::Collection;
use uuid::Uuid;

const DATABASE: &'static str = "test";
const COLLECTION: &'static str = "CoffeeStores";

pub struct MongoCoffeeStoreDao {
    mongo_client: Client
}

impl MongoCoffeeStoreDao {
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
    fn list_stores(self: &Self) -> Result<Vec<CoffeeStoreSummary>, ServerError> {
        log::info!("Listing the coffee stores from MongoDB");
        let mut output = vec![];
        let collection = self.get_coffee_store_collection();
        let cursor = collection.find(None, None)?;
        for document in cursor {
            output.push(from_document(document?)?);
        }

        Ok(output)
    }

    fn create_store(self: &Self, coffee_store: CoffeeStoreManifest) -> Result<CoffeeStoreDetails, ServerError>{
        log::info!("Creating coffee store {:?} and storing in MongoDB", coffee_store);
        let coffee_store_details = CoffeeStoreDetails {
            id: Uuid::new_v4().to_string(),
            name: coffee_store.name,
            description: coffee_store.description,
            avg_rating: coffee_store.avg_rating
        };
        let collection = self.get_coffee_store_collection();
        collection.insert_one(to_document(&coffee_store_details)?, None)?;
        Ok(coffee_store_details)
    }
}