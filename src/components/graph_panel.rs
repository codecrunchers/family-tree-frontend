use crate::call_cytoscape_shim;
use crate::types::{CyElemData, CytoscapeElements};
use serde_json::{json, Result, Value};
use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

use rusted_cypher::cypher::result::{CypherGraphNodeObj, CypherGraphResult};

pub struct GraphPanel {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub family: CypherGraphResult,
}

pub enum Msg {
    Search,
    SetValue(String),
}

impl Component for GraphPanel {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let nodes: Vec<CyElemData> = self
            .props
            .family
            .data
            .clone()
            .iter_mut()
            .flat_map(|g| {
                g.graph
                    .nodes
                    .iter()
                    //                    .filter(|g| g.labels.contains(&"Person".to_string()))
                    .map(|n| CyElemData {
                        data: [
                            ("id".to_owned(), json!(n.id.to_string())),
                            (
                                "name".to_owned(),
                                json!(n
                                    .properties
                                    .get("name")
                                    .unwrap_or(&json!("Family"))
                                    .to_owned()),
                            ),
                        ]
                        .iter()
                        .cloned()
                        .collect(),
                    })
            })
            .collect();

        yew::services::ConsoleService::debug(format!("Nodes {:?}", nodes).as_str());

        let edges: Vec<CyElemData> = self
            .props
            .family
            .data
            .clone()
            .iter_mut()
            .flat_map(move |g| {
                g.graph.relationships.iter().map(|r| {
                    //          yew::services::ConsoleService::debug(format!("g rels {:?}", r).as_str());
                    CyElemData {
                        data: [
                            (
                                "id".to_owned(),
                                json!(format!("{}{}", r.startNode, r.endNode)),
                            ),
                            ("source".to_owned(), json!(r.startNode.clone())),
                            ("target".to_owned(), json!(r.endNode.clone())),
                        ]
                        .iter()
                        .cloned()
                        .collect(),
                    }
                })
            })
            .collect();

        yew::services::ConsoleService::debug(format!("edges {:?}", edges).as_str());

        let cg: CytoscapeElements = CytoscapeElements {
            nodes: nodes,
            edges: edges,
        };

        yew::services::ConsoleService::debug(format!("labels {:?}", cg).as_str());

        html! {
         <>
            <div>{"Tree Representation"}</div>
             <div id="cy"/>
                <button
                    class="msger-send-btn"
                    onclick=self.link.callback(move |_| call_cytoscape_shim( JsValue::from_serde(&cg).unwrap()) )>
                    {"Graph"}
                    </button>
            </>
        }
    }
}
