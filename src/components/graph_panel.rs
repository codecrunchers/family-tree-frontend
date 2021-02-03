use crate::call_cytoscape;
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
        use crate::call_tree;
        html! {
         <>
            <div>{"Tree Representation"}</div>
            <div id="cy"/>
                <button
                    class="msger-send-btn"
                    onclick=self.link.callback(|_| call_cytoscape() )
                />
            </>
        }
    }
}
