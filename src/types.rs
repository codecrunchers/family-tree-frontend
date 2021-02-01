use serde::{Deserialize, Serialize};

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
