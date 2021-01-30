use yew::prelude::*;

pub struct SearchButton {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub on_search: Callback<String>,
}

pub enum Msg {
    Search,
}

impl Component for SearchButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search => {
                let name = "Beth";
                self.props.on_search.emit(name.to_string());
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {

        html! {
            <>
                <input
                    type="text"
                    class="msger-input"
                    id="search-box"
                />
                <button
                    class="msger-send-btn"
                    onclick=self.link.callback(|_| Msg::Search)
                >
                {"Send"}
            </button>
                </>

        }
    }
}
