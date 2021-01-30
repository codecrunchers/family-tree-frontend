use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub price: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Family {
    pub persons: Vec<Person>,
}

