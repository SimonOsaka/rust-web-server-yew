use yew::{function_component, html, Children, Classes, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SectionProps {
    #[prop_or("".into())]
    pub title: String,

    #[prop_or(SectionSize::Default)]
    pub size: SectionSize,

    #[prop_or_default]
    pub children: Children,
}
#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let SectionProps {
        title,
        children,
        size,
    } = props.clone();

    let mut cls = Classes::new();
    cls.push("section");
    cls.push(size);

    html! {
        <section class={cls}>
            <h1 class="title">{ title }</h1>
            <h2 class="subtitle">
                { children.iter().collect::<Html>() }
            </h2>
        </section>
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SectionSize {
    Large,
    Medium,
    Default,
}
impl From<SectionSize> for Classes {
    fn from(ss: SectionSize) -> Self {
        let cls = match ss {
            SectionSize::Large => "is-large",
            SectionSize::Medium => "is-medium",
            SectionSize::Default => "",
        };

        Classes::from(cls)
    }
}
