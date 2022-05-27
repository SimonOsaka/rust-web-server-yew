use crate::components::filter::{Filters, FiltersProps};
use serde_json::json;
use wasm_bindgen::JsValue;
use yew::{function_component, html, Callback};

#[function_component(FiltersPage)]
pub fn filters_page() -> Html {
    let props = FiltersProps {
        data: vec![
            (
                vec![
                    ("Windows".to_string(), "windows".to_string()),
                    ("Macos".to_string(), "macos".to_string()),
                    ("Linux".to_string(), "linux".to_string()),
                ],
                "linux".to_string(),
                "OS".to_string(),
            ),
            (
                vec![
                    ("Intel".to_string(), "intel".to_string()),
                    ("AMD".to_string(), "amd".to_string()),
                ],
                "amd".to_string(),
                "CPU".to_string(),
            ),
            (
                vec![
                    ("1G".to_string(), "1g".to_string()),
                    ("2G".to_string(), "2g".to_string()),
                    ("4G".to_string(), "4g".to_string()),
                    ("8G".to_string(), "8g".to_string()),
                    ("16G".to_string(), "16g".to_string()),
                ],
                "2g".to_string(),
                "MEM".to_string(),
            ),
            (
                vec![
                    ("20G".to_string(), "20g".to_string()),
                    ("40G".to_string(), "40g".to_string()),
                    ("80G".to_string(), "80g".to_string()),
                    ("160G".to_string(), "160g".to_string()),
                    ("512G".to_string(), "512g".to_string()),
                    ("1T".to_string(), "1t".to_string()),
                ],
                "80g".to_string(),
                "DISK".to_string(),
            ),
        ],
        callback: {
            Callback::from(|value: Vec<String>| {
                gloo_console::log!("Filters page ", serde_json::to_string(&value).unwrap());
            })
        },
    };

    html! {
        <Filters ..props />
    }
}
