use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoffeeSummary {
    pub name: String,
    pub avg_rating: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoffeeStore{
    pub name: String,
    pub avg_rating: Option<f32>,
}
