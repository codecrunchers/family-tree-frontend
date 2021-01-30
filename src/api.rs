use crate::types::Person;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn search(name: String, callback: FetchCallback<Vec<Person>>) -> FetchTask {
    let req = Request::get(format!("http://localhost:9090/search/{}", name))
        .body(Nothing)
        .unwrap();
    ConsoleService::debug(format!("Req {:?}", req).as_str());
    FetchService::fetch(req, callback).unwrap()
}
