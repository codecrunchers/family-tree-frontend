use crate::pages::Home;
use crate::route::Route;
use rusted_cypher::cypher::result::CypherGraphResult;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew_router::prelude::*;

//TODO: this needs to be in config
lazy_static::lazy_static! {
    pub static ref ENV_FT_BACKEND: String = String::from("family.snarfel.com");
}

struct State {
    family: CypherGraphResult,
}

pub struct App {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Search(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let family: CypherGraphResult = Default::default();

        Self {
            state: State { family },
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::Search(_name) => true,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        //        let handle_search = self.link.callback(|name: String| Msg::Search(name));
        let family = self.state.family.clone();

        ConsoleService::info(format!("Family = {:?}", family).as_str());

        let render = Router::render(move |switch: Route| match switch {
            Route::HomePage => {
                html! {<Home family=family.clone() , neo_rest_service={ENV_FT_BACKEND.as_str()}  />}
            }
        });

        html! {
            <>
                <Router<Route, ()> render=render/>
            </>
        }
    }
}
