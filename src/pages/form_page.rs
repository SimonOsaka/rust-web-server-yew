use crate::components::{
    button::Button,
    checkbox::Checkbox,
    icon::FontAwesomeIcon,
    input::Input,
    radio::{Radio, RadioGroup},
    select::Select,
    textarea::Textarea,
    Color,
};
use gloo::console;
use serde::Serialize;
use serde_json::json;
use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback};

#[derive(Serialize, Clone, Default)]
pub struct FormValue {
    pub name: String,
    pub username: String,
    pub email: String,
    pub subject: String,
    pub message: String,
    pub agree: bool,
    pub yes: bool,
}

#[function_component(FormPage)]
pub fn form_page() -> Html {
    let form_value = use_state(FormValue::default);
    let radio_value_str = use_state(|| String::from(""));

    // name
    let callback_name = {
        let form_value = form_value.clone();
        Callback::from(move |input_value: String| {
            let mut value = (*form_value).clone();
            value.name = input_value;
            form_value.set(value);
        })
    };

    // username
    let callback_username = {
        let form_value = form_value.clone();
        Callback::from(move |input_value: String| {
            let mut value = (*form_value).clone();
            value.username = input_value;
            form_value.set(value);
        })
    };

    // email
    let callback_email = {
        let form_value = form_value.clone();
        Callback::from(move |input_value: String| {
            let mut value = (*form_value).clone();
            value.email = input_value;
            form_value.set(value);
        })
    };

    // subject
    let callback_subject = {
        let form_value = form_value.clone();
        Callback::from(move |select_value: String| {
            let mut value = (*form_value).clone();
            value.subject = select_value;
            form_value.set(value);
        })
    };

    // message
    let callback_message = {
        let form_value = form_value.clone();
        Callback::from(move |content: String| {
            let mut value = (*form_value).clone();
            value.message = content;
            form_value.set(value);
        })
    };

    // agree
    let callback_agree = {
        let form_value = form_value.clone();
        Callback::from(move |(checked, _)| {
            let mut value = (*form_value).clone();
            value.agree = checked;
            form_value.set(value);
        })
    };

    // yes
    let callback_yes = {
        let form_value = form_value.clone();
        let radio_value_str = radio_value_str.clone();
        Callback::from(move |radio_value: String| {
            let mut value = (*form_value).clone();
            value.yes = matches!(radio_value.as_ref(), "yes");
            radio_value_str.set(radio_value);
            form_value.set(value);
        })
    };

    // submit
    let callback_submit = {
        let form_value = form_value.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let form_value = (*form_value).clone();
            let json_value = json!(form_value);
            console::log!("form value", json_value.to_string());
        })
    };

    html! {
        <>
            <div class="field">
                <label class="label">{"Name"}</label>
                <div class="control">
                    <Input value={(*form_value).name.clone()} placeholder={"Text input"} callback={callback_name} />
                </div>

            </div>

            <div class="field">
                <label class="label">{"Username"}</label>
                <Input value={(*form_value).username.clone()} placeholder={"Username input"} color={Color::Success} control={true} icon_left={true}
                    icon_right={true} callback={callback_username}>
                    <FontAwesomeIcon icon={"fas fa-user"} extra_class={"is-left"} />
                    <FontAwesomeIcon icon={"fas fa-check"} extra_class={"is-right"} />
                </Input>
                <p class="help is-success">{"This username is available"}</p>
            </div>

            <div class="field">
                <label class="label">{"Email"}</label>
                <Input value={(*form_value).email.clone()} placeholder={"Email input"} color={Color::Danger} control={true} icon_left={true}
                    icon_right={true} callback={callback_email}>
                    <FontAwesomeIcon icon={"fas fa-envelope"} extra_class={"is-left"} />
                    <FontAwesomeIcon icon={"fas fa-exclamation-triangle"} extra_class={"is-right"} />
                </Input>
                <p class="help is-danger">{"This email is invalid"}</p>
            </div>

            <div class="field">
                <label class="label">{"Subject"}</label>
                <div class="control">
                    <div class="select">
                        <Select callback={callback_subject}>
                            <option value="" selected={true}>{"Select"}</option>
                            <option value={"Select dropdown"}>{"Select dropdown"}</option>
                            <option value={"With options"}>{"With options"}</option>
                        </Select>
                    </div>
                </div>
            </div>

            <div class="field">
                <label class="label">{"Message"}</label>
                <div class="control">
                    <Textarea value={(*form_value).message.clone()} placeholder={"Textarea"} callback={callback_message} />
                </div>
            </div>

            <div class="field">
                <div class="control">
                    <Checkbox callback={callback_agree}>
                        {" I agree to the "}<a href="#">{"terms and conditions"}</a>
                    </Checkbox>
                </div>
            </div>

            <div class="field">
                <RadioGroup value={(*radio_value_str).clone()} callback={callback_yes}>
                    <Radio label={"Yes"} value={"yes"} />
                    <Radio label={"No"} value={"no"} />
                </RadioGroup>
            </div>

            <div class="field is-grouped">
                <div class="control">
                    <Button color={Color::Link} callback={callback_submit}>{"Submit"}</Button>
                </div>
                <div class="control">
                    <Button color={Color::Link} light={true}>{"Cancel"}</Button>
                </div>
            </div>
        </>
    }
}
