use serde::{Serialize, Deserialize};
use super::common::CoffeeStoreDetails;

/// ## Get Coffee Store API Request.
/// ### id
/// ID of the coffee store to get.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetCoffeeStoreRequest{
    pub coffee_store_id: String,
}

/// ## Get Coffee Store API Response.
/// ### Coffee Store
/// Coffee Stores Details in the response.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetCoffeeStoreResponse{
    pub coffee_store_details: CoffeeStoreDetails,
}
