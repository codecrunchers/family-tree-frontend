#![recursion_limit = "1024"]
extern crate console_error_panic_hook;

mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

use components::html::{LAYOUT, STYLE};
use std::panic;
use types::{Cytoscape, CytoscapeElements};
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

///We can only pass Where T : FromWasmAbi, so some type casting is used here
///prob very slow, need to right this
#[wasm_bindgen]
pub fn call_cytoscape_shim(cg: JsValue) {
    let cgno: CytoscapeElements = JsValue::into_serde(&cg).unwrap();

    let cy: types::Cytoscape = crate::types::Cytoscape {
        autounselectify: false,
        boxSelectionEnabled: true,
        layout: serde_json::from_str(LAYOUT).expect("bad layout man"),
        style: serde_json::from_str(STYLE).expect("bad style man"),
        elements: serde_json::to_value(&cgno).unwrap(),
    };

    let x = render_cytoscape(cy);
    println!("{}", x.is_ok());
}

fn render_cytoscape(c: Cytoscape) -> Result<(), Box<dyn std::error::Error>> {
    let spec_obj = JsValue::from_serde(&c).expect("bad json opts");
    cytoscape_shim(spec_obj);
    Ok(())
}
