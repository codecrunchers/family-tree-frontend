use rusted_cypher::cypher::result::{CypherGraphNode, CypherGraphResult};
use yew::prelude::*;
use yew::services::ConsoleService;
use crate::components::html::bio_panel_view;

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

        let mut first_bio = true;
        let family: Vec<Html> = self
            .props
            .family
            .data
            .clone()
            .iter_mut()
            .flat_map(move |g| {
                g.graph.nodes.iter().map(move |n| {

                    if n.labels.contains(&"Person".to_string()) {
                        let person = n.properties.get("name").unwrap();
                        if first_bio == true {
                            first_bio = false;
                        html! {
                            <div class="carousel-item active">
                                <img src="/imgs/unknown_male.png" class="d-block w-100" alt="..."/>
                                <div class="carousel-caption d-none d-md-block">
                                    <h5>{person}</h5>
                                    <p>{"some text"}</p>
                                </div>
                             </div>
                        }
                        }else{                        
                        html! {
                            <div class="carousel-item">
                               <img src="/imgs/unknown_female.png" class="d-block w-100" alt="..."/>
                                <div class="carousel-caption d-none d-md-block">
                                   <h5>{person}</h5>
                                   <p>{"Nulla vitae elit libero, a pharetra augue mollis interdum."}</p>
                               </div>
                            </div>
                        }
                        }
                    } else {
                        html! {}
                    }
                })
            })
        .collect();
    
        bio_panel_view(family)


    }
}
