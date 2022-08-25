use js_sys::Date;
use rust_web_server_yew::{components::calendar::Calendar, log};
use serde_json::json;
use yew::{function_component, html, Callback};

#[function_component(CalendarPage)]
pub fn calendar_page() -> Html {
    let callback = Callback::from(move |date: String| {
        log!(format!("Calendar page callback date: {}", date));
    });

    let now = Date::now();

    // options: https://doc.mh-s.de/bulma-calendar/#options

    html! {
        <>

            <div class="block">
                <span>{"displayMode: default"}</span>
                <Calendar options={json!({
                    "type": "date",
                    "dateFormat": "yyyy-MM-dd",
                })} callback={callback.clone()} />
            </div>

            <div class="block">
                <span>{"displayMode: dialog"}</span>
                <Calendar options={json!({
                    "type": "date",
                    "dateFormat": "yyyy-MM-dd",
                    "displayMode": "dialog"
                })} callback={callback.clone()} />
            </div>

            <div class="block">
                <span>{"displayMode: inline"}</span>
                <Calendar options={json!({
                    "type": "date",
                    "dateFormat": "yyyy-MM-dd",
                    "displayMode": "inline"
                })} callback={callback.clone()} />
            </div>

            <div class="block">
                <span>{"type: datetime"}</span>
                <Calendar options={json!({
                    "type": "datetime",
                    "dateFormat": "yyyy-MM-dd",
                })} value={Some(now)} callback={callback.clone()} />
            </div>

            <div class="block">
                <span>{"type: time"}</span>
                <Calendar options={json!({"type":"time"})} value={Some(now)} {callback} />
            </div>
        </>
    }
}
