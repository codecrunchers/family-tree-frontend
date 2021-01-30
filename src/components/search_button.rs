use yew::prelude::*;

pub struct SearchButton {
    props: Props,
    value: String,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub on_search: Callback<String>,
}

pub enum Msg {
    Search,
    SetValue(String),
}

impl Component for SearchButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            value: "".to_string(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetValue(v) => {
                self.value = v;
            }
            Msg::Search => {
                let name = self.value.clone();
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
                <input type="text", oninput=self.link.callback(|i: InputData | Msg::SetValue(i.value))/>
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
