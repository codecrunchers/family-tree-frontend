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
pub struct JsTree {
    pub id: String,
    pub treeParams: std::collections::HashMap<String, String>,
}

pub struct Cytoscape {
    container: web_sys::Element,
    autounselectify: bool,
    boxSelectionEnabled: bool,
    layout: Value, //HashMap<String, HashMap<String, String>>,
    style: Value,
    elements: Value,
}
