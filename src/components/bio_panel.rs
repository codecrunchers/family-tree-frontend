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

        let mut count = 0;
        let prev= "Prev";
        let next = "Next";
        let family: Vec<Html> = self
            .props
            .family
            .data
            .clone()
            .iter_mut()
            .flat_map(move |g| {
                count = count + 1;
                g.graph.nodes.iter().map(move |n| {
                    ConsoleService::debug(format!("Node {:?}", n).as_str());

                    if n.labels.contains(&"Person".to_string()) {
                        let person = n.properties.get("name").unwrap();
                        if count == 1 {
                        html! {
                            <div class="carousel-item active">
                                 <img class="d-block w-100" src="https://placeimg.com/1080/500/animals" alt="First slide"/>
            <div class="carousel-caption d-none d-md-block">
                <h5>{next}</h5>
                <p>{prev}</p>
            </div>
                            </div>
                        }
                        }else{                        
                        html! {
                            <div class="carousel-item">
                               <img src="/imgs/unknown_female.png" class="d-block w-100" alt="..."/>
                                <div class="carousel-caption d-none d-md-block">
                                   <h5>{person} {count} {"b"}</h5>
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


        html! {
            <div id="carouselExampleIndicators" class="carousel slide" data-ride="carousel">
    <ol class="carousel-indicators">
        <li data-target="#carouselExampleIndicators" data-slide-to="0" class="active"></li>
        <li data-target="#carouselExampleIndicators" data-slide-to="1"></li>
        <li data-target="#carouselExampleIndicators" data-slide-to="2"></li>
    </ol>
    <div class="carousel-inner">
    {family}
    </div>
    <a class="carousel-control-prev" href="#carouselExampleIndicators" role="button" data-slide="prev">
        <span class="carousel-control-prev-icon" aria-hidden="true"></span>
        <span class="sr-only">{prev}</span>
    </a>
    <a class="carousel-control-next" href="#carouselExampleIndicators" role="button" data-slide="next">
        <span class="carousel-control-next-icon" aria-hidden="true"></span>
        <span class="sr-only">{next}</span>
    </a>
</div>
        }
    }
}
