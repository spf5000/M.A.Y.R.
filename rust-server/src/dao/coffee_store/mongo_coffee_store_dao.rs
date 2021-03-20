use super::CoffeeStoreDao;
use mongodb::sync::Client;
use mongodb::bson::{from_document, to_document, doc};
use rust_server_model::coffee_store::{CoffeeStoreDetails, CoffeeStoreSummary, CoffeeStoreManifest};
use crate::error::{ServerError, ServerErrorType};
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

    fn get_store_by_id(self: &Self, id: &String) -> Result<CoffeeStoreDetails, ServerError> {
        log::info!("Getting coffee store by id {} from MongoDB", id);
        let collection = self.get_coffee_store_collection();
        let filter = doc! { "Id": id };
        let result = collection.find_one(Some(filter), None)?;
        match result {
            Some(document) => Ok(from_document(document)?),
            None => Err(ServerError {
                error_type: ServerErrorType::NotFound(format!("No item with id {} found", id)),
                source: None
            })
        }
    }
}