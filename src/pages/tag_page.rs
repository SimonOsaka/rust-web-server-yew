use crate::components::{
    tag::{GroupTags, Tag, Tags},
    Color,
};
use yew::{function_component, html};

#[function_component(TagPage)]
pub fn tag_page() -> Html {
    html! {
        <>
            <div class="block">
                <Tags>
                    <Tag delete={true}>{"apple"}</Tag>
                    <Tag>{"banana"}</Tag>
                    <Tag>{"orange"}</Tag>
                </Tags>
            </div>

            <div class="block">
                <GroupTags>
                    <Tags addon={true}>
                        <Tag color={Color::Primary}>{"Rust"}</Tag>
                        <Tag>{"1.62.0"}</Tag>
                    </Tags>
                    <Tags addon={true}>
                        <Tag color={Color::Info}>{"Yew"}</Tag>
                        <Tag>{"1.19.3"}</Tag>
                    </Tags>
                    <Tags addon={true}>
                        <Tag color={Color::Dark}>{"Bulma"}</Tag>
                        <Tag>{"0.9.4"}</Tag>
                    </Tags>
                </GroupTags>
            </div>

            <div class="block">
                <GroupTags>
                    <Tags addon={true} delete={true}>
                        <Tag color={Color::Success}>{"apple"}</Tag>
                    </Tags>
                    <Tags addon={true} delete={true}>
                        <Tag color={Color::Danger}>{"orange"}</Tag>
                    </Tags>
                    <Tags addon={true} delete={true}>
                        <Tag color={Color::Link} tag_name={"a".to_string()}>{"banana"}</Tag>
                    </Tags>
                </GroupTags>
            </div>
        </>
    }
}
