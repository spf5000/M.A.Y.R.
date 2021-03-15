use serde::{Serialize, Deserialize};

/// Represents an address for a store.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}
