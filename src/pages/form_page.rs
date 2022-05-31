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
    let onchange_message = {
        let form_value = form_value.clone();
        Callback::from(move |e: Event| {
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
    let onchange_yes = {
        let form_value = form_value.clone();
        Callback::from(move |e: Event| {
            let radio: HtmlInputElement = e.target_unchecked_into();
            let mut value = (*form_value).clone();
            value.yes = match radio.value().as_ref() {
                "yes" => true,
                _ => false,
            };
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
                    <textarea class="textarea" placeholder="Textarea" onchange={onchange_message}></textarea>
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
                <div class="control">
                    <label class="radio">
                        <input type="radio" name="question" value="yes" onchange={onchange_yes.clone()} checked={(*form_value).yes}/>
                        {"Yes"}
                    </label>
                    <label class="radio">
                        <input type="radio" name="question" value="no" onchange={onchange_yes} checked={!(*form_value).yes}/>
                        {"No"}
                    </label>
                </div>
            </div>

            <div class="field is-grouped">
                <div class="control">
                    <button class="button is-link" onclick={onclick_submit}>{"Submit"}</button>
                </div>
                <div class="control">
                    <button class="button is-link is-light">{"Cancel"}</button>
                </div>
            </div>
        </>
    }
}
