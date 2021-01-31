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
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Stock price")
        .description("Google's stock price over time.")
        .data(UrlDataBuilder::default().url(
            "https://raw.githubusercontent.com/davidB/vega_lite_3.rs/master/examples/res/data/stocks.csv"
        ).build()?)
        .transform(vec![
            TransformBuilder::default().filter("datum.symbol==='GOOG'")
        .build()?])
        .mark(Mark::Line)
        .encoding(EncodingBuilder::default()
            .x(XClassBuilder::default()
                .field("date")
                .def_type(StandardType::Temporal)
                .build()?)
            .y(YClassBuilder::default()
                .field("price")
                .def_type(StandardType::Quantitative)
                .build()?).build()?).build()?;
    Ok(chart)
}
