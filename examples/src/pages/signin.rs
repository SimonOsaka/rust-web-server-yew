use rust_web_server_yew::log;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect_with_deps, use_state, Callback, InputEvent, MouseEvent,
    TargetCast,
};
use yew_hooks::use_async;

use crate::{
    api::auth::signin,
    hooks::use_user_context::use_user_context,
    types::auth::{SigninInfo, SigninInfoWrapper},
};

#[function_component(Signin)]
pub fn signin() -> Html {
    let user_ctx = use_user_context();
    let signin_info = use_state(SigninInfo::default);
    let user_signin = {
        let signin_info = signin_info.clone();
        use_async(async move {
            let request = SigninInfoWrapper {
                user: (*signin_info).clone(),
            };
            signin(request).await
        })
    };
    use_effect_with_deps(
        move |user_signin| {
            if let Some(user_info) = &user_signin.data {
                user_ctx.signin(user_info.user.clone());
                log!("signin");
            }
            || ()
        },
        user_signin.clone(),
    );
    let oninput_password = {
        let signup_info = signin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*signup_info).clone();
            info.password = input.value();
            signup_info.set(info);
        })
    };
    let oninput_email = {
        let signup_info = signin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*signup_info).clone();
            info.email = input.value();
            signup_info.set(info);
        })
    };
    let onclick = {
        let login_info = signin_info.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let s = &*login_info;
            log!(&s.password, &s.email);
            user_signin.run();
        })
    };
    html! {
        <>

            <div class="field">
                <label class="label">{"Email"}</label>
                <div class="control has-icons-left has-icons-right">
                    <input class="input is-danger" type="email" placeholder="Email input"
                        value={signin_info.email.clone()} oninput={oninput_email} />
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
                <label class="label">{"Password"}</label>
                <div class="control">
                    <input class="input" type="password" placeholder="Password input"
                        value={signin_info.password.clone()} oninput={oninput_password} />
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
