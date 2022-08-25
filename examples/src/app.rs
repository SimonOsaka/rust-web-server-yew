use crate::pages::{
    card_list::CardList, example::Example, home::Home, navbar::Navbar,
    page_not_found::PageNotFound, signin::Signin, signup::Signup,
    user_context_provider::UserContextProvider,
};
use rust_web_server_yew::components::{
    footer::Footer, loading::Loadings, notifications::Notifications,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/example")]
    Example,
    #[at("/contact")]
    Contact,
    #[at("/about")]
    About,
    #[at("/signup")]
    Signup,
    #[at("/signin")]
    Signin,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <BrowserRouter>
                <Navbar/>

                <main class="container">
                    <Switch<Route> render={Switch::render(switch)} />
                    <Notifications />
                    <Loadings />
                </main>

                <Footer/>
            </BrowserRouter>
        </UserContextProvider>
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Contact => {
            html! { <CardList/> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::Example => {
            html! { <Example /> }
        }
        Route::About => {
            html! {}
        }
        Route::Signup => {
            html! { <Signup /> }
        }
        Route::Signin => {
            html! { <Signin /> }
        }
    }
}
