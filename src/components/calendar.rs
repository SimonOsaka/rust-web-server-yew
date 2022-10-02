use crate::{components::gen_auto_id, log};
use gloo::utils::format::JsValueSerdeExt;
use serde_json::{json, Value};
use std::fmt::Display;
use wasm_bindgen::prelude::*;
use yew::{classes, function_component, html, use_mut_ref, Callback, Properties};
use yew_hooks::use_effect_once;

#[wasm_bindgen(module = "/javascript/bulma-calendar.js")]
extern "C" {
    type BulmaCalendar;

    #[wasm_bindgen(constructor)]
    fn new(id: JsValue) -> BulmaCalendar;

    #[wasm_bindgen(method)]
    fn init(this: &BulmaCalendar, options: JsValue, default_value: JsValue);

    #[wasm_bindgen(method)]
    fn on_select(this: &BulmaCalendar, callback: &Closure<dyn Fn(JsValue)>);

}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CalendarProps {
    #[prop_or_else(gen_auto_id)]
    pub id: String,

    #[prop_or(json!({}))]
    pub options: Value,

    /// milliseconds
    #[prop_or(None)]
    pub value: Option<f64>,

    #[prop_or_default]
    pub callback: Callback<String>,
}

#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    let CalendarProps {
        id,
        callback,
        value,
        options,
    } = props.clone();

    let calendar = { use_mut_ref(|| BulmaCalendar::new(JsValue::from_str(&id))) };

    {
        use_effect_once(move || {
            let jsvalue = <JsValue as JsValueSerdeExt>::from_serde(&options);
            if let Ok(options) = jsvalue {
                calendar.borrow_mut().init(
                    options,
                    match value {
                        Some(t) => JsValue::from_f64(t),
                        None => JsValue::NULL,
                    },
                );

                let cb = Closure::wrap(Box::new(move |e: JsValue| {
                    log!(format!(
                        "calendar callback: {}",
                        e.as_string().unwrap_or_default()
                    ));
                    callback.emit(e.as_string().unwrap_or_default());
                }) as Box<dyn Fn(JsValue)>);

                calendar.borrow_mut().on_select(&cb);

                cb.forget();
            }

            || ()
        });
    }

    html! {
        <div>
            <input class={classes!(format!("calendar_{}",id))} type="text" />
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CalendarType {
    Datetime,
    Date,
    Time,
}
impl Display for CalendarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CalendarType::Datetime => "datetime",
            CalendarType::Date => "date",
            CalendarType::Time => "time",
        })
    }
}
