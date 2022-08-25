use crate::{components::gen_auto_id, log};
use serde_json::{json, Value};
use yew::{function_component, html, use_state, Properties};

use wasm_bindgen::prelude::*;
use yew_hooks::{use_effect_once, use_effect_update_with_deps};

#[wasm_bindgen(module = "/javascript/chart.js")]
extern "C" {
    type MyChart;

    #[wasm_bindgen(constructor)]
    fn new(id: JsValue) -> MyChart;

    #[wasm_bindgen(method)]
    fn draw(this: &MyChart, config: JsValue);

    #[wasm_bindgen(method)]
    fn update(this: &MyChart, data: JsValue);
}

fn get_chart_data<T>(label: Vec<String>, data: Vec<T>, title: String) -> Value
where
    T: serde::ser::Serialize,
{
    let label = label.as_slice();
    let data = data.as_slice();
    json!({
        "labels": label,
        "datasets": [{
            "label": title,
            "backgroundColor": "rgb(255, 99, 132)",
            "borderColor": "rgb(255, 99, 132)",
            "data": data,
        }]
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct LineChartProps {
    #[prop_or_else(gen_auto_id)]
    pub id: String,
    #[prop_or(Vec::<String>::new())]
    pub label: Vec<String>,
    #[prop_or(Vec::<i32>::new())]
    pub data: Vec<i32>,
    #[prop_or("My Title".to_string())]
    pub title: String,
    #[prop_or(400)]
    pub width: u32,
    #[prop_or(400)]
    pub height: u32,
}

#[function_component(LineChart)]
pub fn line_chart(props: &LineChartProps) -> Html {
    let LineChartProps {
        data,
        label,
        title,
        id,
        width,
        height,
    } = props.clone();

    let chart = {
        let id = id.clone();
        use_state(|| MyChart::new(JsValue::from_str(id.as_str())))
    };

    {
        let data = data.clone();
        let chart = chart.clone();
        let label = label.clone();
        let title = title.clone();
        use_effect_once(move || {
            log!("line_chart => use_effect_once");
            let data = get_chart_data(label.clone(), data.clone(), title);
            let config = json!({
                "type": "line",
                "data": data,
                "options": {
                    "responsive": false
                }
            });
            let jsvalue = JsValue::from_serde(&config);
            if let Ok(cfg) = jsvalue {
                (*chart).draw(cfg);
            }
            || ()
        });
    }

    {
        use_effect_update_with_deps(
            move |data| {
                log!("line_chart => use_effect_update_with_deps");
                let data = get_chart_data(label.clone(), data.clone(), title);
                let jsvalue = JsValue::from_serde(&data);
                if let Ok(d) = jsvalue {
                    (*chart).update(d);
                }
                || ()
            },
            data,
        );
    }

    html! {
        <div>
            <canvas {id} width={width.to_string()} height={height.to_string()}></canvas>
        </div>
    }
}
