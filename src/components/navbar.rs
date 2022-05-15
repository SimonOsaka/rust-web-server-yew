use crate::{app::Route, hooks::use_user_context::use_user_context, types::auth::UserInfo};
use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback, Html};
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let user_ctx = use_user_context();

    let onclick_logout = {
        let user_ctx = user_ctx.clone();
        Callback::from(move |_e: MouseEvent| {
            user_ctx.signout();
        })
    };

    let user_html = {
        if user_ctx.is_authenticated() {
            login_html((*user_ctx).clone(), onclick_logout)
        } else {
            logout_html()
        }
    };

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <a class="navbar-item" href="https://bulma.io">
                    <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28" />
                </a>

                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false"
                    data-target="navbarBasicExample">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div id="navbarBasicExample" class="navbar-menu">
                <div class="navbar-start">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>

                    <Link<Route> classes={classes!("navbar-item")} to={Route::Documentation}>
                        { "Documentation" }
                    </Link<Route>>

                    <div class="navbar-item has-dropdown is-hoverable">
                        <a class="navbar-link">
                            {"More"}
                        </a>

                        <div class="navbar-dropdown">
                            <Link<Route> classes={classes!("navbar-item")} to={Route::About}>
                                { "About" }
                            </Link<Route>>
                            <a class="navbar-item">
                                {"Jobs"}
                            </a>
                            <Link<Route> classes={classes!("navbar-item")} to={Route::Contact}>
                                { "Contact" }
                            </Link<Route>>
                            <hr class="navbar-divider" />
                            <a class="navbar-item">
                                {"Report an issue"}
                            </a>
                        </div>
                    </div>
                </div>

                { user_html }
            </div>
        </nav>
    }
}

fn login_html(userinfo: UserInfo, onclick_logout: Callback<MouseEvent>) -> Html {
    html! {
        <div class="navbar-end">
            <div class="navbar-item">
                <div class="buttons">
                    <a class="button is-text">
                        <strong>{ &userinfo.username }</strong>
                    </a>
                    <a class="button is-light" onclick={onclick_logout}>
                        {"Log out"}
                    </a>
                </div>
            </div>
        </div>
    }
}

fn logout_html() -> Html {
    html! {
        <div class="navbar-end">
            <div class="navbar-item">
                <div class="buttons">
                    <Link<Route> classes={classes!("button", "is-primary")} to={Route::Signup}>
                        <strong>{"Sign up"}</strong>
                    </Link<Route>>
                    <Link<Route> classes={classes!("button", "is-light")} to={Route::Signin}>
                        {"Sign in"}
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}
