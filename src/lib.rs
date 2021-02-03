mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

use futures_timer::Delay;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use vega_lite_3::*;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::*;
use web_sys::Element;
use yew::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
use types::{Cytoscape, JsTree};
use yew::services::ConsoleService;

#[wasm_bindgen(start)]
pub fn run_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook)); //this ain't doin much
    App::<app::App>::new().mount_to_body();
}

//Start Vega Stuff
//

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn vegaEmbed(target: web_sys::Element, spec: JsValue, option: JsValue) -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    fn treeMaker(spec: JsValue, option: JsValue);
}

#[wasm_bindgen]
extern "C" {
    fn cytoscape(spec: JsValue);
}

#[wasm_bindgen]
pub fn call_cytoscape() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document
        .get_element_by_id("cy")
        .expect("not cy dom element found");

    let cy: types::Cytoscape = crate::types::Cytoscape {
        container: val,
        autounselectify: true,
        boxSelectionEnabled: false,
        layout: serde_json::from_str(r##"{ "layout":{"name":"cola"}}"##).expect("bad layout man "),
        style: serde_json::from_str(r##"{"style":[{"selector":"node","css":{"background-color":"#f92411"}},{"selector":"edge","css":{"line-color":"#f92411"}}]}"##).expect("badstyleman"),
        elements: serde_json::from_str(r##"{
      "nodes":[
         {
            "data":{
               "id":"1",
               "label":"P"
            }
         },
         {
            "data":{
               "id":"2",
               "label":"sucrose phosphate phosphatase"
            }
         },
         {
            "data":{
               "source":"1",
               "target":"2"
            }
         }
      ]
    }}"##).expect("bad nodes man"),
};

    render_cytoscape(cy);
}

fn render_cytoscape(c: Cytoscape) -> Result<(), Box<dyn std::error::Error>> {
    let spec_obj = JsValue::from_serde(&c).expect("bad json opts");
    cytoscape(spec_obj);
    Ok(())
}

#[wasm_bindgen]
pub fn call_tree() {
    let tree = r##"{1: 'myTree'}"##;
    let tree_opts = r##"{id: 'myTree', treeParams: { 1 : {'trad': 'Alan'}}}"##;
    render_chart_1(tree.to_string(), tree_opts.to_string()).expect("to render it");
}

/// Render chart onto the web_sys::Element, with optional dict, allow resize if a container web_sys::Element is provided.
pub fn render_chart_1(tree: String, tree_opts: String) -> Result<(), JsValue> /*Box<dyn std::error::Error>>*/
{
    let tree = r##"
    {
       "1":{
          "2":"",
          "3":{
             "6":"",
             "7":""
          },
          "4":"",
          "5":""
       }
    }"##;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(tree).expect("bad tree json");

    let mut names: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    names.insert("1".to_string(), "Alan".to_string());
    names.insert("2".to_string(), "Barry".to_string());
    names.insert("3".to_string(), "Sarah".to_string());
    names.insert("4".to_string(), "Dad".to_string());
    names.insert("5".to_string(), "Mam".to_string());
    names.insert("6".to_string(), "Lou".to_string());
    names.insert("7".to_string(), "Puppy".to_string());

    let js_tree: JsTree = JsTree {
        id: "myTree".to_string(),
        treeParams: names,
    };
    let spec = JsValue::from_serde(&v).expect("bad json");
    ConsoleService::debug(format!("spec for tree = {:?}", spec).as_str());
    let spec_opts = JsValue::from_serde(&js_tree).expect("bad json opts");
    ConsoleService::debug(format!("spec opts for tree = {:?}", spec_opts).as_str());
    treeMaker(spec, spec_opts);
    Ok(())
}

/// Render chart onto the web_sys::Element, with optional dict, allow resize if a container web_sys::Element is provided.
pub fn render_chart(
    chart: &Vegalite,
    target: web_sys::Element,
    option: &Option<HashMap<String, String>>,
    watch_resize: Option<web_sys::Element>,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = JsValue::from_serde(chart)?;
    let opt = match &option {
        Some(x) => JsValue::from_serde(x)?,
        None => JsValue::from(js_sys::Object::new()),
    };
    let fut: JsFuture = vegaEmbed(target, spec, opt).into();

    if let Some(target) = watch_resize {
        let target: web_sys::HtmlElement = target.unchecked_into();
        spawn_local(async move {
            let res = fut.await.unwrap();
            let view = js_sys::Reflect::get(&res, &JsValue::from_str("view")).unwrap();
            let mut dimension = [0_i32; 2];
            let width = js_sys::Function::from(
                js_sys::Reflect::get(&view, &JsValue::from_str("width")).unwrap(),
            );
            let height = js_sys::Function::from(
                js_sys::Reflect::get(&view, &JsValue::from_str("height")).unwrap(),
            );
            let run = js_sys::Function::from(
                js_sys::Reflect::get(&view, &JsValue::from_str("run")).unwrap(),
            );
            loop {
                Delay::new(Duration::from_millis(100)).await;
                let new_dimension = [target.offset_width(), target.offset_height()];
                if (dimension != new_dimension) && new_dimension != [0, 0] {
                    dimension = new_dimension;
                    width.call1(&view, &JsValue::from(dimension[0])).unwrap();
                    height.call1(&view, &JsValue::from(dimension[1])).unwrap();
                    run.call0(&view).unwrap();
                }
            }
        });
    }

    Ok
}
