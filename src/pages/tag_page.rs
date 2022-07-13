use crate::components::tag::{GroupTags, Tag, TagColors, Tags};
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

            <GroupTags>
                <Tags addon={true}>
                    <Tag color={TagColors::Primary}>{"Rust"}</Tag>
                    <Tag>{"1.62.0"}</Tag>
                </Tags>
                <Tags addon={true}>
                    <Tag color={TagColors::Info}>{"Yew"}</Tag>
                    <Tag>{"1.19.3"}</Tag>
                </Tags>
                <Tags addon={true}>
                    <Tag color={TagColors::Dark}>{"Bulma"}</Tag>
                    <Tag>{"0.9.4"}</Tag>
                </Tags>
            </GroupTags>

            <GroupTags>
                <Tags addon={true} delete={true}>
                    <Tag color={TagColors::Success}>{"apple"}</Tag>
                </Tags>
                <Tags addon={true} delete={true}>
                    <Tag color={TagColors::Danger}>{"orange"}</Tag>
                </Tags>
                <Tags addon={true} delete={true}>
                    <Tag color={TagColors::Link} tag_name={"a".to_string()}>{"banana"}</Tag>
                </Tags>
            </GroupTags>
        </>
    }
}
