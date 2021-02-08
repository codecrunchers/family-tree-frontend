use crate::call_cytoscape_shim;
use serde_json::json;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use rusted_cypher::cypher::result::{CypherGraphNode, CypherGraphResult};

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
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        use crate::types::CytoscapeElements;
        use crate::{get_edges, get_nodes};
        use wasm_bindgen::prelude::*;

        let nodes = get_nodes(&mut self.props.family.data.clone());
        let edges = get_edges(&mut self.props.family.data.clone());

        let cg: CytoscapeElements = CytoscapeElements {
            nodes: nodes,
            edges: edges,
        };
        let uid_for_image_hack = js_sys::Date::now();
        yew::html! {
        <>
            <div id="cy"/>
            <img src={uid_for_image_hack} onerror=self
                .link
                .callback(move |_| call_cytoscape_shim(JsValue::from_serde(&cg).unwrap()))/>

        </>
        }
    }
}
