use crate::components::{
    button::{Button, ButtonColors},
    radio::{Radio, RadioGroup},
};
use serde::Serialize;
use serde_json::json;
use web_sys::{
    Event, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, InputEvent, MouseEvent,
};
use yew::{function_component, html, use_state, Callback, NodeRef, TargetCast};
use yew_hooks::use_effect_once;

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
    let oninput_name = {
        let form_value = form_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut value = (*form_value).clone();
            value.name = input.value();
            form_value.set(value);
        })
    };

    // username
    let oninput_username = {
        let form_value = form_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut value = (*form_value).clone();
            value.username = input.value();
            form_value.set(value);
        })
    };

    // email
    let oninput_email = {
        let form_value = form_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut value = (*form_value).clone();
            value.email = input.value();
            form_value.set(value);
        })
    };

    let node_ref_select = NodeRef::default();

    // subject
    let onchange_subject = {
        let form_value = form_value.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let mut value = (*form_value).clone();
            value.subject = select.value();
            form_value.set(value);
        })
    };

    // message
    let oninput_message = {
        let form_value = form_value.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            let mut value = (*form_value).clone();
            value.message = textarea.value();
            form_value.set(value);
        })
    };

    // agree
    let onchange_agree = {
        let form_value = form_value.clone();
        Callback::from(move |e: Event| {
            let checkbox: HtmlInputElement = e.target_unchecked_into();
            let mut value = (*form_value).clone();
            value.agree = checkbox.checked();
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
    let onclick_submit = {
        let form_value = form_value.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let fv = (*form_value).clone();
            let jv = json!(fv);
            gloo_console::log!("form value", jv.to_string());
        })
    };

    {
        let node_ref_select = node_ref_select.clone();
        use_effect_once(move || {
            if let Some(node_select) = node_ref_select.cast::<HtmlSelectElement>() {
                node_select.set_selected_index(0);
            };
            || ()
        })
    };

    html! {
        <>
            <div class="field">
                <label class="label">{"Name"}</label>
                <div class="control">
                    <input class="input" type="text" placeholder="Text input" oninput={oninput_name}/>
                </div>
            </div>

            <div class="field">
                <label class="label">{"Username"}</label>
                <div class="control has-icons-left has-icons-right">
                    <input class="input is-success" type="text" placeholder="Text input" oninput={oninput_username}/>
                    <span class="icon is-small is-left">
                        <i class="fas fa-user"></i>
                    </span>
                    <span class="icon is-small is-right">
                        <i class="fas fa-check"></i>
                    </span>
                </div>
                <p class="help is-success">{"This username is available"}</p>
            </div>

            <div class="field">
                <label class="label">{"Email"}</label>
                <div class="control has-icons-left has-icons-right">
                    <input class="input is-danger" type="email" placeholder="Email input" oninput={oninput_email}/>
                    <span class="icon is-small is-left">
                        <i class="fas fa-envelope"></i>
                    </span>
                    <span class="icon is-small is-right">
                        <i class="fas fa-exclamation-triangle"></i>
                    </span>
                </div>
                <p class="help is-danger">{"This email is invalid"}</p>
            </div>

            <div class="field">
                <label class="label">{"Subject"}</label>
                <div class="control">
                    <div class="select">
                        <select ref={node_ref_select} onchange={onchange_subject}>
                            <option value="">{"Select"}</option>
                            <option value={"Select dropdown"}>{"Select dropdown"}</option>
                            <option value={"With options"}>{"With options"}</option>
                        </select>
                    </div>
                </div>
            </div>

            <div class="field">
                <label class="label">{"Message"}</label>
                <div class="control">
                    <textarea class="textarea" placeholder="Textarea" oninput={oninput_message}></textarea>
                </div>
            </div>

            <div class="field">
                <div class="control">
                    <label class="checkbox">
                        <input type="checkbox" onchange={onchange_agree} checked={(*form_value).agree}/>
                        {" I agree to the "}<a href="#">{"terms and conditions"}</a>
                    </label>
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
                    <Button color={ButtonColors::Link} callback={onclick_submit}>{"Submit"}</Button>
                </div>
                <div class="control">
                    <Button color={ButtonColors::Link} light={true}>{"Cancel"}</Button>
                </div>
            </div>
        </>
    }
}
