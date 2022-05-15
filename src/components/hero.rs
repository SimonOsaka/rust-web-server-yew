use yew::{function_component, html};

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="hero">
            <div class="hero-body">
                <p class="title">
                {"Hero title"}
                </p>
                <p class="subtitle">
                {"Hero subtitle"}
                </p>
            </div>
        </section>
    }
}
