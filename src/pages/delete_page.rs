use crate::components::{delete::Delete, tag::Tag, Size};
use web_sys::MouseEvent;
use yew::{function_component, html, Callback};

#[function_component(DeletePage)]
pub fn delete_page() -> Html {
    let callback = Callback::from(|e: MouseEvent| {
        e.prevent_default();
        gloo_console::log!("click");
    });

    html! {
        <>
            <div class="block">
                <Delete />
            </div>

            <div class="block">
                <Delete size={Size::Large} />
            </div>

            <div class="block">
                <Tag>
                    {"Html"}
                    <Delete size={Size::Small} {callback} />
                </Tag>
            </div>
        </>
    }
}
