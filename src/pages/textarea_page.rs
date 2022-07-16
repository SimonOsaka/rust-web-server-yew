use crate::components::textarea::Textarea;
use yew::{function_component, html, Callback};

use crate::components::textarea::{TextareaColors, TextareaSizes};

#[function_component(TextareaPage)]
pub fn textarea_page() -> Html {
    let callback = Callback::from(|str: String| {
        gloo_console::log!(str);
    });

    html! {
        <>
            <Textarea placeholder={"placeholder"} {callback} />

            <Textarea value={"i'm value"} />

            <Textarea value={"i'm value"} disable={true} />

            <Textarea value={"i'm value"} readonly={true} />

            <Textarea color={TextareaColors::Primary} placeholder={"Primary color"} />

            <Textarea size={TextareaSizes::Large} placeholder={"Large size"}/>

            <Textarea placeholder={"placeholder"} control={true} loading={true} />
        </>
    }
}
