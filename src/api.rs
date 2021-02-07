use anyhow::Error;
use rusted_cypher::cypher::result::CypherGraphResult;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

const API_SVC_HOST: &'static str = "192.168.1.8";

pub fn search(name: String, callback: FetchCallback<CypherGraphResult>) -> FetchTask {
    let req = Request::get(format!("http://{}:9090/search/{}", API_SVC_HOST, name))
        .body(Nothing)
        .unwrap();
    ConsoleService::debug(format!("Req {:?}", req).as_str());
    FetchService::fetch(req, callback).unwrap()
}

pub fn family(callback: FetchCallback<CypherGraphResult>) -> FetchTask {
    let req = Request::get(format!("http://{}:9090/family", API_SVC_HOST))
        .body(Nothing)
        .unwrap();
    ConsoleService::debug(format!("Req {:?}", req).as_str());
    FetchService::fetch(req, callback).unwrap()
}
