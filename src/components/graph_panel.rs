use crate::call_cytoscape_shim;
use crate::types::{CyElemData, CytoscapeElements};
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
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
             <div id="cy"/>
        }
    }
}
