use crate::types::Person;
use yew::prelude::*;

pub struct BioPanel {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub family: Vec<Person>,
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
        let family: Vec<Html> = self
            .props
            .family
            .iter()
            .map(|person: &Person| {
                html! {
                <div class="product_card_container">
                    <div  classes="product_card_anchor">
                        <div class="product_card_name">{&person.name}</div>
                        <div class="product_card_name">{&person.bio}</div>
                        <div class="product_card_name">{person.pid}</div>
                    </div>
                </div>
                    }
            })
            .collect();

        html! {

            <div class="product_card_list">{family}</div>
        }
    }
}
