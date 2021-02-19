use anyhow::Error;
use rusted_cypher::cypher::result::CypherGraphResult;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn search(
    name: String,
    callback: FetchCallback<CypherGraphResult>,
    api_svc_host: String,
) -> FetchTask {
    let req = Request::get(format!("http://{}:9090/search/{}", api_svc_host, name))
        .body(Nothing)
        .unwrap();
    ConsoleService::log(format!("Req {:?}", req).as_str());
    FetchService::fetch(req, callback).unwrap()
}

pub fn family(callback: FetchCallback<CypherGraphResult>, api_svc_host: String) -> FetchTask {
    ConsoleService::info(format!("Target Host {}", api_svc_host).as_str());
    let req = Request::get(format!("http://{}:9090/family", api_svc_host))
        .body(Nothing)
        .unwrap();
    ConsoleService::log(format!("Req {:?}", req).as_str());
    FetchService::fetch(req, callback).unwrap()
}
