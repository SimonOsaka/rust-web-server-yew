use rust_web_server_yew::components::{markdown::MarkdownPreview, textarea::Textarea};
use yew::{function_component, html, use_state, Callback};

#[function_component(MarkDownPage)]
pub fn markdown_page() -> Html {
    let value = use_state(|| "".to_string());

    let callback = {
        let value = value.clone();
        Callback::from(move |content: String| {
            value.set(content);
        })
    };

    html! {
        <div class="columns">
            <div class="column is-half">
                <Textarea rows={12} placeholder={"Markdown content"} value={(*value).clone()} {callback} />
            </div>
            <div class="column is-half">
                <MarkdownPreview value={(*value).to_string()} />
            </div>
        </div>
    }
}
