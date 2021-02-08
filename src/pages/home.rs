use crate::api;
use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};

use crate::components::html::{home_loading, home_view};
use crate::components::{BioPanel, GraphPanel, SearchButton};
use anyhow::Error;
use rusted_cypher::cypher::result::CypherGraphResult;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

struct State {
    family: CypherGraphResult,
    get_search_error: Option<Error>,
    get_search_loaded: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub family: CypherGraphResult,
}

pub struct Home {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum SearchType {
    ByName,
    FAMILY,
}

pub enum Msg {
    GetSearch(SearchType, Option<String>),
    GetSearchSuccess(CypherGraphResult),
    GetSearchError(Error),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetSearch(SearchType::FAMILY, None));

        Self {
            props,
            state: State {
                family: Default::default(),
                get_search_error: None,
                get_search_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetSearch(search_type, name) => match search_type {
                SearchType::ByName => {
                    self.state.get_search_loaded = false;
                    let handler = self.link.callback(
                        move |response: api::FetchResponse<CypherGraphResult>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(family) => Msg::GetSearchSuccess(family),
                                Err(err) => Msg::GetSearchError(err),
                            }
                        },
                    );
                    ConsoleService::info(
                        format!("Searching for {}", name.clone().unwrap()).as_str(),
                    );

                    let name = name.unwrap();
                    let iter = utf8_percent_encode(&name, FRAGMENT);
                    let encoded: String = iter.collect();

                    self.task = Some(api::search(encoded, handler.clone()));
                    false
                }
                SearchType::FAMILY => {
                    self.state.get_search_loaded = false;

                    let handler = self.link.callback(
                        move |response: api::FetchResponse<CypherGraphResult>| {
                            //ConsoleService::debug(format!("Json Response {:?}", response).as_str());
                            let (_, Json(data)) = response.into_parts();
                            //ConsoleService::debug(format!("Json Parts {:?}", data).as_str());
                            match data {
                                Ok(family) => Msg::GetSearchSuccess(family),
                                Err(err) => Msg::GetSearchError(err),
                            }
                        },
                    );

                    ConsoleService::info("Searching for Family");
                    self.task = Some(api::family(handler));
                    false
                }
            },
            Msg::GetSearchSuccess(cypher_result) => {
                //ConsoleService::debug(format!("cypher_result {:?}", cypher_result).as_str());
                let family: CypherGraphResult = cypher_result;
                self.state.family = family;
                //ConsoleService::debug(format!("Family {:?}", self.state.family).as_str());
                self.state.get_search_loaded = true;
                true
            }
            Msg::GetSearchError(error) => {
                ConsoleService::error(format!("error {:?}", error).as_str());
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
        let search_handler = self
            .link
            .callback(|name: String| Msg::GetSearch(SearchType::ByName, Some(name)));

        if !self.state.get_search_loaded {
            home_loading()
        } else {
            home_view(self.state.family.clone(), search_handler.clone())
        }
    }
}
