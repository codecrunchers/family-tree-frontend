use crate::api;
use crate::types::Person;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
    family: Vec<Person>,
    get_search_error: Option<Error>,
    get_search_loaded: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub family: Vec<Person>,
    pub on_search: Callback<String>,
}

pub struct Home {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    GetSearch(String),
    GetSearchSuccess(Vec<Person>),
    GetSearchError(Error),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let family = vec![];

        link.send_message(Msg::GetSearch("Beth".to_string()));

        Self {
            props,
            state: State {
                family,
                get_search_error: None,
                get_search_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetSearch(name) => {
                self.state.get_search_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Vec<Person>>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(family) => Msg::GetSearchSuccess(family),
                                Err(err) => Msg::GetSearchError(err),
                            }
                        });

                self.task = Some(api::search(name, handler));
                false
            }
            Msg::GetSearchSuccess(family) => {
                self.state.family = family;
                self.state.get_search_loaded = true;
                true
            }
            Msg::GetSearchError(error) => {
                self.state.get_search_error = Some(error);
                self.state.get_search_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let family: Vec<Html> = vec![];

        if !self.state.get_search_loaded {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else if let Some(error) = &self.state.get_search_error {
            html! {
              <div>
                <span>{"Error loading products! :("}</span>
                <div>{error}</div>
              </div>
            }
        } else {
            html! {
                <div class="product_card_list">{family}</div>
            }
        }
    }
}
