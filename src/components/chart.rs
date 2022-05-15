use serde_json::{json, Value};
use yew::{html, Component, Context, Html};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(module = "/javascript/mychart.js")]
extern "C" {
    type MyChart;

    #[wasm_bindgen(constructor)]
    fn new() -> MyChart;

    #[wasm_bindgen(method)]
    fn draw(this: &MyChart, config: JsValue);

    #[wasm_bindgen(method)]
    fn update(this: &MyChart, data: JsValue);
}

const LABELS: [&str; 6] = ["January", "February", "March", "April", "May", "June"];

pub enum Msg {
    Draw,
}

pub struct Chart {
    chart: MyChart,
    is_clicked: bool,
}

impl Component for Chart {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            chart: MyChart::new(),
            is_clicked: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                if !self.is_clicked {
                    let data = get_chart_data(vec![30, 25, 20, 15, 10, 5]);
                    let jsvalue = JsValue::from_serde(&data);
                    if let Ok(d) = jsvalue {
                        self.chart.update(d);
                    }
                    self.is_clicked = true;
                    console_log!("self.is_clicked: {}", self.is_clicked);
                } else {
                    console_log!("chart was drawn");
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <>
                <button onclick={link.callback(|_| Msg::Draw)}>{ "update" }</button>
                <div>
                    <canvas id="mychart" width="400" height="400"></canvas>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let data = get_chart_data(vec![0, 5, 10, 15, 20, 25]);
            let config = json!({
                "type": "line",
                "data": data,
                "options": {
                    "responsive": false
                }
            });
            let jsvalue = JsValue::from_serde(&config);
            if let Ok(cfg) = jsvalue {
                self.chart.draw(cfg);
            }
        }
    }
}

fn get_chart_data<T>(data: Vec<T>) -> Value
where
    T: serde::ser::Serialize,
{
    let data = data.as_slice();
    json!({
        "labels": LABELS,
        "datasets": [{
            "label": "My First dataset",
            "backgroundColor": "rgb(255, 99, 132)",
            "borderColor": "rgb(255, 99, 132)",
            "data": data,
        }]
    })
}
