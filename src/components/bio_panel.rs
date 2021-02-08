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
        //ConsoleService::debug(format!("Bio Change {:?}", props.family).as_str());
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        //ConsoleService::debug(format!("bio data {:?}", self.props.family).as_str());

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

        ConsoleService::debug(format!("family: {:?}", family.len()).as_str());
        ConsoleService::debug(format!("family Obj: {:?}", family).as_str());
        family.sort_by(|a, b| b.id.cmp(&a.id));
        family.dedup();
        ConsoleService::debug(format!("family: {:?}", family.len()).as_str());
        ConsoleService::debug(format!("family  Obj: {:?}", family).as_str());

        let family: Vec<Html> = family.iter().map(|n| bio_panel_bio(n)).collect();
        /*let f1 = self.props.family.data.clone();
        let family: Vec<Html> = f1
            .iter()
            .flat_map(|g| {
                g.graph
                    .nodes
                    .iter()
                    .filter(move |g| g.labels.contains(&"Person".to_string()))
                    .map(move |n| {
                        ConsoleService::debug(
                            format!("Processing Node {:?}", n.properties.get("fullName")).as_str(),
                        );
                        //ConsoleService::debug(format!("Cur Dupe List {:?}", dd1).as_str())

                        let id = n.properties.get("person_id").unwrap();
                        ConsoleService::debug(format!("Person: {}", id).as_str());
                        bio_panel_bio(n)
                    })
            })
            .collect();*/

        bio_panel_view(family)
    }
}
