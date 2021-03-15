use serde::{Serialize, Deserialize};
use super::common::{CoffeeStoreDetails, CoffeeStoreManifest};

/// ## Create Coffee Stores API Request.
/// ### Coffee Store
/// Details of the coffee store to create.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCoffeeStoreRequest{
    pub coffee_store: CoffeeStoreManifest
}

/// ## Get Coffee Stores API Response.
/// ### Coffee Store
/// Coffee Stores in the response.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCoffeeStoreResponse{
    pub coffee_store_details: CoffeeStoreDetails,
}
