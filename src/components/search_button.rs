use crate::types::Product;
use yew::prelude::*;

pub struct SearchButton {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: Product,
    pub on_search: Callback<Product>,
}

pub enum Msg {
    Search,
}

impl Component for AtcButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search => self.props.on_search.emit(self.props.product.clone()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::Search;

        html! {
          <button class="product_search_button" onclick=onclick>{"Search"}</button>
        }
    }
}
