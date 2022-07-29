use gloo::console;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect_with_deps, use_state, Callback, InputEvent, MouseEvent,
    TargetCast,
};
use yew_hooks::use_async;
use yew_router::{history::History, hooks::use_history};

use crate::{
    api::auth::signup,
    app::Route,
    types::auth::{SignupInfo, SignupInfoWrapper},
};

#[function_component(Signup)]
pub fn sign_up() -> Html {
    let history = use_history().unwrap();
    let signup_info = use_state(SignupInfo::default);
    let user_signup = {
        let signup_info = signup_info.clone();
        use_async(async move {
            let request = SignupInfoWrapper {
                user: (*signup_info).clone(),
            };
            console::log!("signup request");
            signup(request).await
        })
    };
    use_effect_with_deps(
        move |user_signup| {
            if user_signup.data.is_some() {
                console::log!("signup to signin");
                history.push(Route::Signin);
            }
            console::log!("signup");
            || ()
        },
        user_signup.clone(),
    );
    let oninput_username = {
        let signup_info = signup_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*signup_info).clone();
            info.username = input.value();
            signup_info.set(info);
        })
    };
    let oninput_password = {
        let signup_info = signup_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*signup_info).clone();
            info.password = input.value();
            signup_info.set(info);
        })
    };
    let oninput_email = {
        let signup_info = signup_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*signup_info).clone();
            info.email = input.value();
            signup_info.set(info);
        })
    };
    let onclick = {
        let signup_info = signup_info.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let s = &*signup_info;
            console::log!(&s.username, &s.password, &s.email);
            user_signup.run();
        })
    };
    html! {
        <>

            <div class="field">
                <label class="label">{"Username"}</label>
                <div class="control has-icons-left has-icons-right">
                    <input class="input is-success" type="text" placeholder="Username input"
                        value={signup_info.username.clone()} oninput={oninput_username} />
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
                <label class="label">{"Password"}</label>
                <div class="control">
                    <input class="input" type="password" placeholder="Password input" value={signup_info.password.clone()}
    oninput={oninput_password} />
                </div>
            </div>

            <div class="field">
                <label class="label">{"Email"}</label>
                <div class="control has-icons-left has-icons-right">
                    <input class="input is-danger" type="email" placeholder="Email input" value={signup_info.email.clone()} oninput={oninput_email} />
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
                <div class="control">
                    <label class="checkbox">
                        <input type="checkbox" />
                        {"I agree to the "}<a href="#">{"terms and conditions"}</a>
                    </label>
                </div>
            </div>

            <div class="field is-grouped">
                <div class="control">
                    <button class="button is-link" onclick={onclick}>{"Submit"}</button>
                </div>
            </div>

        </>
    }
}
