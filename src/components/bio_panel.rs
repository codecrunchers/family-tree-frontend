use crate::call_cytoscape_shim;
use crate::components::html::{bio_panel_bio, bio_panel_view};
use rusted_cypher::cypher::result::{CNode, CypherGraphNode, CypherGraphResult};
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct BioPanel {
    props: Props,
    value: String, //seatch key
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub family: CypherGraphResult,
    pub on_search: Callback<String>,
}

pub enum Msg {
    Search,
    SetValue(String),
}

impl Component for BioPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            value: "".into(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
        /*        match msg {
            Msg::SetValue(v) => {
                self.value = v;
            }
            Msg::Search => {
                let name = self.value.clone();
                self.props.on_search.emit(name.to_string());
            }
        }
        true*/
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        ConsoleService::debug(format!("Bio Change {:?}", props.family).as_str());
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let f1 = self.props.family.data.clone();
        let mut family: Vec<_> = f1
            .iter()
            .flat_map(|g| {
                g.graph
                    .nodes
                    .iter()
                    .filter(move |g| g.labels.contains(&"Person".to_string()))
                    .map(move |n| n)
            })
            .collect();

        family.sort_by(|a, b| b.id.cmp(&a.id));
        family.dedup();
        let family: Vec<Html> = family.iter().map(|n| bio_panel_bio(n)).collect();

        use crate::types::CytoscapeElements;
        use crate::{get_edges, get_nodes};
        use wasm_bindgen::prelude::*;

        let nodes = get_nodes(&mut self.props.family.data.clone());
        let edges = get_edges(&mut self.props.family.data.clone());

        let cg: CytoscapeElements = CytoscapeElements {
            nodes: nodes,
            edges: edges,
        };

        let on_change_handler = self
            .link
            .callback(move |_| call_cytoscape_shim(JsValue::from_serde(&cg).unwrap()));
        bio_panel_view(family, on_change_handler)
    }
}
