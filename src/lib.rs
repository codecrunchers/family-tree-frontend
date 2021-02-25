use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn cytoscape_shim(spec: JsValue);
}
