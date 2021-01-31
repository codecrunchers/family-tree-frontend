use crate::types::Person;
use yew::prelude::*;

pub struct GraphPanel {
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
        <div id="container" style="width: 100vw; height: 100vh">
            <div id="viz"></div>
        </div>
        }
    }
}
