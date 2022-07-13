use crate::components::tag::{Tag, TagColors, Tags};
use yew::{function_component, html};

#[function_component(TagPage)]
pub fn tag_page() -> Html {
    html! {
        <>
            <Tags>
                <Tag delete={true}>{"apple"}</Tag>
                <Tag>{"banana"}</Tag>
                <Tag>{"orange"}</Tag>
            </Tags>

            <Tags addon={true}>
                <Tag color={TagColors::Primary}>{"Rust"}</Tag>
                <Tag>{"1.62.0"}</Tag>
            </Tags>

            <Tags addon={true} delete={true}>
                <Tag color={TagColors::Danger}>{"orange"}</Tag>
            </Tags>

            <Tags addon={true} delete={true}>
                <Tag color={TagColors::Link} tag_name={"a".to_string()}>{"banana"}</Tag>
            </Tags>
        </>
    }
}
