use serde::{Serialize, Deserialize};

/// Represents a coffee store with the high level information about that store.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CoffeeStoreSummary{
    pub id: String,
    pub name: String,
    pub avg_rating: Option<f32>,
}

/// Represents a coffee store with the detailed information about that store.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CoffeeStoreDetails{
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub avg_rating: Option<f32>,
}

/// Represents the details needed to create a coffee store.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CoffeeStoreManifest{
    pub name: String,
    pub description: Option<String>,
    pub avg_rating: Option<f32>,
}

