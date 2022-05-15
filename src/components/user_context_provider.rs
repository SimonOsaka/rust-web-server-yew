//! User context provider.

use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

use crate::{
    api::{auth::current, get_token, set_token},
    error::Error,
    types::auth::UserInfo,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);
    let current_user = use_async(async move { current().await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                gloo_console::log!("token exist");
                current_user.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(user_info) = &current_user.data {
                    // fake u
                    // let u = UserInfo {
                    //     username: "u1s2".to_string(),
                    //     token: "token123".to_string(),
                    //     ..Default::default()
                    // };
                    user_ctx.set(user_info.user.clone());
                    // user_ctx.set(u);
                }
                gloo_console::log!("rendered");
                if let Some(error) = &current_user.error {
                    match error {
                        Error::Unauthorized | Error::Forbidden => set_token(None),
                        _ => (),
                    }
                }
                || ()
            },
            current_user,
        )
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
