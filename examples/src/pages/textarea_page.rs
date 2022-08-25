use rust_web_server_yew::{
    components::{textarea::Textarea, Color, Size},
    log,
};
use yew::{function_component, html, Callback};

#[function_component(TextareaPage)]
pub fn textarea_page() -> Html {
    let callback = Callback::from(|str: String| {
        log!(str);
    });

    html! {
       <>
            <div class="block">
               <Textarea placeholder={"placeholder"} {callback} />
            </div>

            <div class="block">
                <Textarea value={"i'm value"} />
            </div>

            <div class="block">
                <Textarea value={"i'm value"} disable={true} />
            </div>

            <div class="block">
                <Textarea value={"i'm value"} readonly={true} />
            </div>

            <div class="block">
                <Textarea color={Color::Primary} placeholder={"Primary color"} />
            </div>

            <div class="block">
                <Textarea size={Size::Large} placeholder={"Large size"}/>
            </div>

            <div class="block">
                <Textarea placeholder={"placeholder"} control={true} loading={true} />
            </div>
        </>
    }
}
