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
    let spec = r##"{
  "$schema": "https://vega.github.io/schema/vega/v5.json",
  "description": "An example of Cartesian layouts for a node-link diagram of hierarchical data.",
  "width": 600,
  "height": 1600,
  "padding": 5,

  "signals": [
    {
      "name": "labels", "value": true,
      "bind": {"input": "checkbox"}
    },
    {
      "name": "layout", "value": "tidy",
      "bind": {"input": "radio", "options": ["tidy", "cluster"]}
    },
    {
      "name": "links", "value": "diagonal",
      "bind": {
        "input": "select",
        "options": ["line", "curve", "diagonal", "orthogonal"]
      }
    },
    {
      "name": "separation", "value": false,
      "bind": {"input": "checkbox"}
    }
  ],

  "data": [
    {
      "name": "tree",
      "url": "data/flare.json",
      "transform": [
        {
          "type": "stratify",
          "key": "id",
          "parentKey": "parent"
        },
        {
          "type": "tree",
          "method": {"signal": "layout"},
          "size": [{"signal": "height"}, {"signal": "width - 100"}],
          "separation": {"signal": "separation"},
          "as": ["y", "x", "depth", "children"]
        }
      ]
    },
    {
      "name": "links",
      "source": "tree",
      "transform": [
        { "type": "treelinks" },
        {
          "type": "linkpath",
          "orient": "horizontal",
          "shape": {"signal": "links"}
        }
      ]
    }
  ],

  "scales": [
    {
      "name": "color",
      "type": "linear",
      "range": {"scheme": "magma"},
      "domain": {"data": "tree", "field": "depth"},
      "zero": true
    }
  ],

  "marks": [
    {
      "type": "path",
      "from": {"data": "links"},
      "encode": {
        "update": {
          "path": {"field": "path"},
          "stroke": {"value": "#ccc"}
        }
      }
    },
    {
      "type": "symbol",
      "from": {"data": "tree"},
      "encode": {
        "enter": {
          "size": {"value": 100},
          "stroke": {"value": "#fff"}
        },
        "update": {
          "x": {"field": "x"},
          "y": {"field": "y"},
          "fill": {"scale": "color", "field": "depth"}
        }
      }
    },
    {
      "type": "text",
      "from": {"data": "tree"},
      "encode": {
        "enter": {
          "text": {"field": "name"},
          "fontSize": {"value": 9},
          "baseline": {"value": "middle"}
        },
        "update": {
          "x": {"field": "x"},
          "y": {"field": "y"},
          "dx": {"signal": "datum.children ? -7 : 7"},
          "align": {"signal": "datum.children ? 'right' : 'left'"},
          "opacity": {"signal": "labels ? 1 : 0"}
        }
      }
    }
  ]
}
"##;
    let chart: Vegalite = serde_json::from_str(spec)?;
    Ok(chart)
}
