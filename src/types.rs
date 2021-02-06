use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::Element;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QueryResponse {
    pub columns: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Person {
    pub pid: i32,
    pub name: String,
    pub bio: String,
    //    pub image: String,
    //    pub dod: String,
    pub dob: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Cytoscape {
    pub autounselectify: bool,
    pub boxSelectionEnabled: bool,
    pub layout: Value,
    pub style: Value,
    pub elements: Value,
}
