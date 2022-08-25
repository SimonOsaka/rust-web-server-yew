use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section class="hero is-link is-fullheight-with-navbar">
            <div class="hero-body">
                <p class="title">
                    {"Welcome back home!"}
                </p>
            </div>
        </section>
    }
}
