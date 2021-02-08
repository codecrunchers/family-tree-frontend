use crate::components::html::{bio_panel_bio, bio_panel_view};
use rusted_cypher::cypher::result::{CNode, CypherGraphNode, CypherGraphResult};
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct BioPanel {
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

impl Component for BioPanel {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        ConsoleService::debug(format!("Bio Change {:?}", props.family).as_str());
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        //        ConsoleService::debug(format!("bio data {:?}", self.props.family).as_str());

        let mut f1 = self.props.family.data.clone();

        let family: Vec<Html> = f1
            .iter_mut()
            .flat_map(|g| {
                ConsoleService::debug(format!("Graph: {:?}", g).as_str());
                g.graph
                    .nodes
                    .iter()
                    .filter(|g| g.labels.contains(&"Person".to_string()))
                    .map(|n| bio_panel_bio(n))
            })
            .collect();

        bio_panel_view(family)
    }
}
