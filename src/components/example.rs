use crate::render_chart;
use std::collections::HashMap;
use vega_lite_3::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;

#[wasm_bindgen]
pub fn call_vega() {
    //    femme::start(log::LevelFilter::Info).unwrap();
    let doc = web_sys::window().unwrap().document().unwrap();
    let target = doc.get_element_by_id("viz").unwrap();

    if let Ok(chart) = gen_chart() {
        let mut option = HashMap::<String, String>::new();
        option.insert("renderer".to_string(), "svg".to_string());
        let doc = web_sys::window()
            .map(|win| win.document())
            .flatten()
            .unwrap();
        let container = doc.get_element_by_id("container").unwrap();
        render_chart(&chart, target, &Some(option), Some(container)).expect("to render it");
    }
}

pub fn gen_chart() -> Result<Vegalite, Box<dyn std::error::Error>> {
    let spec = r##"
    "$schema": "https://vega.github.io/schema/vega/v5.json",
  "width": 200,
  "height": 100,
  "padding": 5,

  "signals": [
    { "name": "method", "value": "tidy",
      "bind": {"input": "select", "options": ["tidy", "cluster"]} },
    { "name": "separation", "value": true, "bind": {"input": "checkbox"} }
  ],

  "data": [
    {
      "name": "tree",
      "values": [
        {"id": "A", "parent": null},
        {"id": "B", "parent": "A"},
        {"id": "C", "parent": "A"},
        {"id": "D", "parent": "C"},
        {"id": "E", "parent": "C"}
      ],
      "transform": [
        {
          "type": "stratify",
          "key": "id",
          "parentKey": "parent"
        },
        {
          "type": "tree",
          "method": {"signal": "method"},
          "separation": {"signal": "separation"},
          "size": [{"signal": "width"}, {"signal": "height"}]
        }
      ]
    },
    {
      "name": "links",
      "source": "tree",
      "transform": [
        { "type": "treelinks" },
        { "type": "linkpath" }
      ]
    }
  ],

  "scales": [
    {
      "name": "color",
      "type": "ordinal",
      "range": {"scheme": "category20"}
    }
  ],

  "marks": [
    {
      "type": "path",
      "from": {"data": "links"},
      "encode": {
        "enter": {
          "stroke": {"value": "#ccc"}
        },
        "update": {
          "path": {"field": "path"}
        }
      }
    },
    {
      "type": "symbol",
      "from": {"data": "tree"},
      "encode": {
        "enter": {
          "fill": {"scale": "color", "field": "id"},
          "stroke": {"value": "white"},
          "size": {"value": 400}
        },
        "update": {
          "x": {"field": "x"},
          "y": {"field": "y"}
        }
      }
    }
  ]
}##";

let chart: Vegalite = serde_json::from_str(spec)?;
    Ok(chart)
}
