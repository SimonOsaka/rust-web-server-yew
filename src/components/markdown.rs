use gloo::utils::document;
use pulldown_cmark::{html::push_html, Options, Parser};
use web_sys::{Element, Node};
use yew::{function_component, html, Html, Properties};

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct MarkdownProps {
    #[prop_or("".into())]
    pub value: String,
}
#[function_component(MarkdownPreview)]
pub fn markdown_preview(props: &MarkdownProps) -> Html {
    let MarkdownProps { value } = props.clone();

    let markdown_html = parse_text(&value);
    let div: Element = document().create_element("div").unwrap();
    div.set_class_name("content");
    div.set_inner_html(&markdown_html);
    let node: Node = div.into();
    let preview = Html::VRef(node);

    html! {
        { preview }
    }
}

fn parse_text(value: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(value, options);
    let mut parsed_text = String::new();
    push_html(&mut parsed_text, parser);

    parsed_text
}
