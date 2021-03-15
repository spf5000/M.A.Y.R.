use serde::{Serialize, Deserialize};
use super::common::CoffeeStoreSummary;

/// ## List Coffee Stores API Request.
/// ### Max Items
/// maximum items returned by the API. **Cannot exceed 10 items.** Default value is 10 items if not provided.
/// ### Next Token
/// Token to start the List Coffee Stores request from. Should be acquired from a previous response.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ListCoffeeStoresRequest{
    pub max_items: Option<String>,
    pub next_token: Option<String>,
}

/// ## List Coffee Stores API Response.
/// ### Coffee Stores
/// Coffee Stores in the response.
/// ### Next Token
/// Token to start the List Coffee Stores request from for future calls.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ListCofeeStoresResponse{
    pub coffee_stores: Vec<CoffeeStoreSummary>,
    pub next_token: Option<String>
}
