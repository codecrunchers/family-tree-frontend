use yew::prelude::*;

pub struct SearchButton {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub name: String,
    pub on_search: Callback<Option<String>>,
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
            Msg::Search => self.props.on_search.emit(Some(self.props.name.clone())),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::Search);
        html! {
          <button class="product_search_button" onclick=onclick>{"Search"}</button>

        }
    }
}
