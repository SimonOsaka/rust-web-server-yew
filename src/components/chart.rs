use serde_json::{json, Value};
use yew::Component;
use yew::{function_component, html, use_state, Properties};

use wasm_bindgen::prelude::*;
use yew_hooks::{use_effect_once, use_effect_update_with_deps};

#[wasm_bindgen(module = "/javascript/mychart.js")]
extern "C" {
    type MyChart;

    #[wasm_bindgen(constructor)]
    fn new(id: JsValue) -> MyChart;

    #[wasm_bindgen(method)]
    fn draw(this: &MyChart, config: JsValue);

    #[wasm_bindgen(method)]
    fn update(this: &MyChart, data: JsValue);
}

// pub enum Msg {
//     Draw,
// }

// pub struct Chart {
//     chart: MyChart,
//     is_clicked: bool,
// }

// impl Component for Chart {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {
//             chart: MyChart::new(),
//             is_clicked: false,
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::Draw => {
//                 if !self.is_clicked {
//                     let data = get_chart_data(vec![30, 25, 20, 15, 10, 5]);
//                     let jsvalue = JsValue::from_serde(&data);
//                     if let Ok(d) = jsvalue {
//                         self.chart.update(d);
//                     }
//                     self.is_clicked = true;
//                     console_log!("self.is_clicked: {}", self.is_clicked);
//                 } else {
//                     console_log!("chart was drawn");
//                 }

//                 true
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
//         let link = ctx.link();
//         html! {
//             <>
//                 <button onclick={link.callback(|_| Msg::Draw)}>{ "update" }</button>
//                 <div>
//                     <canvas id="mychart" width="400" height="400"></canvas>
//                 </div>
//             </>
//         }
//     }

//     fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
//         if first_render {
//             let data = get_chart_data(vec![0, 5, 10, 15, 20, 25]);
//             let config = json!({
//                 "type": "line",
//                 "data": data,
//                 "options": {
//                     "responsive": false
//                 }
//             });
//             let jsvalue = JsValue::from_serde(&config);
//             if let Ok(cfg) = jsvalue {
//                 self.chart.draw(cfg);
//             }
//         }
//     }
// }

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
    #[prop_or("my_linechart".to_string())]
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
    } = props;

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
            gloo_console::log!("line_chart => use_effect_once");
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
        let data = data.clone();
        let chart = chart.clone();
        let label = label.clone();
        let title = title.clone();
        use_effect_update_with_deps(
            move |data| {
                gloo_console::log!("line_chart => use_effect_update_with_deps");
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

    let chart_id = {
        let id = id.clone();
        id
    };

    html! {
        <div>
            <canvas id={chart_id} width={width.to_string()} height={height.to_string()}></canvas>
        </div>
    }
}
