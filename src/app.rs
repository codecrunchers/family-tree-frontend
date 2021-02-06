use crate::components::GraphPanel;
use crate::pages::Home;
use crate::route::Route;
use crate::types::Person;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew_router::prelude::*;

struct State {
    family: Vec<Person>,
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
        let family = vec![];

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
        let handle_search = self.link.callback(|name: String| Msg::Search(name));
        let family = self.state.family.clone();

        ConsoleService::info(format!("Family = {:?}", family).as_str());

        let render = Router::render(move |switch: Route| match switch {
            Route::HomePage => {
                html! {<Home family=family.clone()  />}
            }
        });

        html! {
            <>
                <Router<Route, ()> render=render/>
            </>
        }
    }
}
