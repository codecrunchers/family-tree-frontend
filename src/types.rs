use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::Element;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QueryResponse {
    pub columns: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FamilyMember {
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Cytoscape {
    pub autounselectify: bool,
    pub boxSelectionEnabled: bool,
    pub layout: Value,
    pub style: Value,
    pub elements: Value,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CytoscapeElements {
    pub nodes: Vec<CyElemData>,
    pub edges: Vec<CyElemData>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CyElemData {
    pub data: std::collections::HashMap<String, String>,
}
