use rust_web_server_yew::{
    components::{delete::Delete, tag::Tag, Size},
    log,
};
use web_sys::MouseEvent;
use yew::{function_component, html, Callback};

#[function_component(DeletePage)]
pub fn delete_page() -> Html {
    let callback = Callback::from(|e: MouseEvent| {
        e.prevent_default();
        log!("click");
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
