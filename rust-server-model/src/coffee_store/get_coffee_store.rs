use serde::{Serialize, Deserialize};
use super::common::CoffeeStoreDetails;

/// ## Get Coffee Stores API Request.
/// ### id
/// ID of the coffee store to get.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetCoffeeStoresRequest{
    pub coffee_store_id: String,
}

/// ## Get Coffee Stores API Response.
/// ### Coffee Store
/// Coffee Stores Details in the response.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetCofeeStoresResponse{
    pub coffee_store_details: CoffeeStoreDetails,
}
