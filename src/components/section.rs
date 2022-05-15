use yew::{function_component, html};

#[function_component(Section)]
pub fn section() -> Html {
    html! {
        <section class="section">
            <h1 class="title">{"Section"}</h1>
            <h2 class="subtitle">
                {"A simple container to divide your page into "}<strong>{"sections"}</strong>{", like the one you're currently reading."}
            </h2>
        </section>
    }
}
