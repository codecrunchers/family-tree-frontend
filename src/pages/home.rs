use crate::api;
use crate::components::{BioPanel, GraphPanel, SearchButton};
use crate::types::Person;
use anyhow::Error;
use rusted_cypher::cypher::result::{CypherResult, Row};
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;

struct State {
    family: Vec<Person>,
    get_search_error: Option<Error>,
    get_search_loaded: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub family: Vec<Person>,
}

pub struct Home {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum SearchType {
    by_name,
    family,
}

pub enum Msg {
    GetSearch(SearchType, Option<String>),
    GetSearchSuccess(CypherResult),
    GetSearchError(Error),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let family = vec![];

        link.send_message(Msg::GetSearch(SearchType::family, None));

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
            Msg::GetSearch(search_type, name) => match search_type {
                SearchType::by_name => {
                    self.state.get_search_loaded = false;
                    let handler =
                        self.link
                            .callback(move |response: api::FetchResponse<CypherResult>| {
                                ConsoleService::debug(
                                    format!("Json Response {:?}", response).as_str(),
                                );
                                let (_, Json(data)) = response.into_parts();
                                ConsoleService::debug(format!("Json Parts {:?}", data).as_str());
                                match data {
                                    Ok(family) => Msg::GetSearchSuccess(family),
                                    Err(err) => Msg::GetSearchError(err),
                                }
                            });
                    ConsoleService::info(
                        format!("Searching for {}", name.clone().unwrap()).as_str(),
                    );
                    self.task = Some(api::search(name.unwrap(), handler.clone()));
                    false
                }
                SearchType::family => {
                    self.state.get_search_loaded = false;

                    let handler =
                        self.link
                            .callback(move |response: api::FetchResponse<CypherResult>| {
                                ConsoleService::debug(
                                    format!("Json Response {:?}", response).as_str(),
                                );
                                let (_, Json(data)) = response.into_parts();
                                ConsoleService::debug(format!("Json Parts {:?}", data).as_str());
                                match data {
                                    Ok(family) => Msg::GetSearchSuccess(family),
                                    Err(err) => Msg::GetSearchError(err),
                                }
                            });

                    ConsoleService::info("Searching for Family");
                    self.task = Some(api::family(handler));
                    false
                }
            },
            Msg::GetSearchSuccess(cypher_result) => {
                let rows: Vec<Row> = cypher_result.rows().collect();
                let family: Vec<Person> = rows
                    .iter()
                    .map(|row| Person {
                        pid: row.get::<i32>("p.pid").unwrap(),
                        name: row.get::<String>("p.name").unwrap(),
                        bio: row.get::<String>("p.bio").unwrap(),
                        //                        image: row.get::<String>("p.image").unwrap(),
                        dob: row.get::<String>("p.dob").unwrap(),
                        //                      dod: row.get::<String>("p.dod").unwrap(),
                    })
                    .collect();

                self.state.family = family;

                ConsoleService::debug(format!("Family {:?}", self.state.family).as_str());
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
        let search_handler = self
            .link
            .callback(|name: String| Msg::GetSearch(SearchType::by_name, Some(name)));

        if !self.state.get_search_loaded {
            html! {
            <>
                            <SearchButton on_search=search_handler.clone() />
                            <div class="loading_spinner_container">
                                <div class="loading_spinner"></div>
                                <div class="loading_spinner_text">{"Loading ..."}</div>
                            </div>
                            </>
                        }
        } else if let Some(error) = &self.state.get_search_error {
            html! {
                <>
                <SearchButton on_search=search_handler.clone() />
              <div>
                <span>{"Error loading products! :("}</span>
                <div>{error}</div>
              </div>
              </>
            }
        } else {
            html! {
              <>
                  <SearchButton on_search=search_handler.clone() />
                  <BioPanel family=self.state.family.clone()/>
                  <GraphPanel family=self.state.family.clone()/>
                  </>
            }
        }
    }
}
