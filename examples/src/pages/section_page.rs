use rust_web_server_yew::components::section::{Section, SectionSize};
use yew::{function_component, html};

#[function_component(SectionPage)]
pub fn section_page() -> Html {
    html! {
        <>

            <Section title={"Section"}>
                {"A simple container to divide your page into"} <strong>{"sections"}</strong>{", like the one you're
                currently reading."}
            </Section>

            <Section title={"Section"} size={SectionSize::Medium}>
                {"A simple container to divide your page into"} <strong>{"sections"}</strong>{", like the one you're
                currently reading."}
            </Section>

            <Section title={"Section"} size={SectionSize::Large}>
                {"A simple container to divide your page into"} <strong>{"sections"}</strong>{", like the one you're
                currently reading."}
            </Section>

        </>
    }
}
