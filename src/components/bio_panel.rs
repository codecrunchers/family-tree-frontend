use rusted_cypher::cypher::result::{CypherGraphNode, CypherGraphResult};
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
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        ConsoleService::debug(format!("bio data {:?}", self.props.family).as_str());

        let family: Vec<u32> = self
            .props
            .family
            .data
            .clone()
            .iter_mut()
            .flat_map(|g| g.graph.nodes.iter().map(|n| 1))
            .collect();

        /*        let family: Vec<Html> = self.props.family.data.clone().iter_mut().flat_map(|g| {
                    let family = g
                        .graph
                        .nodes
                        .into_iter()
                        .flat_map(|n| {
                            html! {
                            <div class="product_card_container">
                                <div  classes="product_card_anchor">
                                    <div class="product_card_name">{n.properties.get("name").unwrap()}</div>
                                </div>
                            </div>
                            }
                        }).collect()
                }).collect();
        */

        html! {

            <div class="product_card_list">{family}</div>
        }
    }
}
