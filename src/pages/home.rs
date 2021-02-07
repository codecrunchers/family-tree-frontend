use crate::api;
use crate::components::{BioPanel, GraphPanel, SearchButton};
use anyhow::Error;
use rusted_cypher::cypher::result::CypherGraphResult;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;

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
                            ConsoleService::debug(format!("Json Response {:?}", response).as_str());
                            let (_, Json(data)) = response.into_parts();
                            ConsoleService::debug(format!("Json Parts {:?}", data).as_str());
                            match data {
                                Ok(family) => Msg::GetSearchSuccess(family),
                                Err(err) => Msg::GetSearchError(err),
                            }
                        },
                    );
                    ConsoleService::info(
                        format!("Searching for {}", name.clone().unwrap()).as_str(),
                    );
                    self.task = Some(api::search(name.unwrap(), handler.clone()));
                    false
                }
                SearchType::FAMILY => {
                    self.state.get_search_loaded = false;

                    let handler = self.link.callback(
                        move |response: api::FetchResponse<CypherGraphResult>| {
                            ConsoleService::debug(format!("Json Response {:?}", response).as_str());
                            let (_, Json(data)) = response.into_parts();
                            ConsoleService::debug(format!("Json Parts {:?}", data).as_str());
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
                ConsoleService::debug(format!("cypher_result {:?}", cypher_result).as_str());
                let family: CypherGraphResult = cypher_result;
                self.state.family = family;
                ConsoleService::debug(format!("Family {:?}", self.state.family).as_str());
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
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else {
            html! {
                             <>
                                 <script src="https://code.jquery.com/jquery-3.2.1.slim.min.js" integrity="sha384-KJ3o2DKtIkvYIK3UENzmM7KCkRr/rE9/Qpg6aAZGJwFDMVNA/GpGFF93hXpG5KkN" crossorigin="anonymous"></script>
            <script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.12.9/umd/popper.min.js" integrity="sha384-ApNbgh9B+Y1QKtv3Rn7W3mgPxhU9K/ScQsAP7hUibX39j7fakFPskvXusvfa0b4Q" crossorigin="anonymous"></script>
            <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0/js/bootstrap.min.js" integrity="sha384-JZR6Spejh4U02d8jOt6vLEHfe/JQGiRRSQQxSfFWpi1MquVdAyjUar5+76PVCmYl" crossorigin="anonymous"></script>

                                 <div class="container">
                               <div class="row">
                                 <div class="col-md-8">

                                 <div class="card mb-4">
                                     <div class="card-body">
                                        <SearchButton on_search=search_handler.clone() />
                                     </div>
                                   </div>

                                   <div class="card mb-4">
                                     <div class="card-body">
                                           <BioPanel family=self.state.family.clone()/>
                                     </div>
                                   </div>

                                   <div class="card mb-4">
                                     <div class="card-body">
                                        <GraphPanel family=self.state.family.clone()/>
                                     </div>
                                   </div>

                                </div>
                             </div>
                           </div>

                           </>
                               }
        }
    }
}
