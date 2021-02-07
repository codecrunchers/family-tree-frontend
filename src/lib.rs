#![recursion_limit = "256"]
extern crate console_error_panic_hook;

mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

use std::panic;
use types::Cytoscape;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook)); //this ain't doin much
    App::<app::App>::new().mount_to_body();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn cytoscape_shim(spec: JsValue);
}

#[wasm_bindgen]
pub fn call_cytoscape_shim(spec: String) {
    let cy: types::Cytoscape = crate::types::Cytoscape {
        autounselectify: false,
        boxSelectionEnabled: true,
        layout: serde_json::from_str(
            r##"{
               "name":"cola",
               "convergenceThreshold":100,
               "animate":false
            }"##,
        )
        .expect("bad layout man"),
        style: serde_json::from_str(
            r##"[
            {
             "selector": "node",
              "style": {
               "label": "data(id)"
               }
              },
              {
                "selector": "edge",
                "css": {
                   "line-color": "#f92411"
                }
               }
              ]"##,
        )
        .expect("bad style man"),
        elements: serde_json::from_str(
            r##"{
       "nodes":[
          {
             "data":{
                "id":"A",
                "name":"A"
             }
          },
          {
             "data":{
                "id":"B",
                "name":"B"
             }
          },
          {
             "data":{
                "id":"C"
             }
          },
          {
             "data":{
                "id":"D"
             }
          },
          {
             "data":{
                "id":"E"
             }
          },
          {
             "data":{
                "id":"F"
             }
          },
          {
             "data":{
                "id":"G"
             }
          },
          {
             "data":{
                "id":"H"
             }
          },
          {
             "data":{
                "id":"J"
             }
          },
          {
             "data":{
                "id":"K"
             }
          },
          {
             "data":{
                "id":"L"
             }
          },
          {
             "data":{
                "id":"M"
             }
          }
       ],
       "edges":[
          {
             "data":{
                "id":"e1",
                "source":"A",
                "target":"B"
             }
          },
          {
             "data":{
                "id":"e2",
                "source":"A",
                "target":"C"
             }
          },
          {
             "data":{
                "id":"e3",
                "source":"B",
                "target":"D"
             }
          },
          {
             "data":{
                "id":"e4",
                "source":"C",
                "target":"D"
             }
          },
          {
             "data":{
                "id":"e5",
                "source":"C",
                "target":"E"
             }
          },
          {
             "data":{
                "id":"e6",
                "source":"C",
                "target":"F"
             }
          },
          {
             "data":{
                "id":"e7",
                "source":"D",
                "target":"G"
             }
          },
          {
             "data":{
                "id":"e8",
                "source":"D",
                "target":"H"
             }
          },
          {
             "data":{
                "id":"e9",
                "source":"E",
                "target":"H"
             }
          },
          {
             "data":{
                "id":"e10",
                "source":"E",
                "target":"J"
             }
          },
          {
             "data":{
                "id":"e11",
                "source":"F",
                "target":"J"
             }
          },
          {
             "data":{
                "id":"e12",
                "source":"F",
                "target":"K"
             }
          },
          {
             "data":{
                "id":"e13",
                "source":"G",
                "target":"L"
             }
          },
          {
             "data":{
                "id":"e14",
                "source":"H",
                "target":"L"
             }
          },
          {
             "data":{
                "id":"e15",
                "source":"H",
                "target":"M"
             }
          },
          {
             "data":{
                "id":"e16",
                "source":"J",
                "target":"M"
             }
          }
       ]
}"##,
        )
        .expect("bad nodes man"),
    };

    let x = render_cytoscape(cy);
    println!("{}", x.is_ok());
}

fn render_cytoscape(c: Cytoscape) -> Result<(), Box<dyn std::error::Error>> {
    let spec_obj = JsValue::from_serde(&c).expect("bad json opts");
    cytoscape_shim(spec_obj);
    Ok(())
}
