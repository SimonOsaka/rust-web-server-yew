use crate::components::{
    footer::Footer, navbar::Navbar, notifications::Notifications,
    user_context_provider::UserContextProvider,
};
use crate::pages::{
    card_list::CardList, documentation::Documentation, home::Home, page_not_found::PageNotFound,
    signin::Signin, signup::Signup,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/documentation")]
    Documentation,
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
                    // <BreadCrumb data={ [("Bulma".to_string(),false),("Documentation".to_string(),false),("Components".to_string(),false),("Breadcrumb".to_string(),true)].to_vec()}/>
                    // <CardList/>

                    // <Section/>

                    // <Hero/>

                    // <MediaObjectArticle/>
                    // <MediaObjectPost/>
                    // <MediaObjectNestPost/>




                    // <ModalImage is_active=false/>
                    // <ModalCard is_active=false/>
                    <Notifications />
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
        Route::Documentation => {
            html! { <Documentation /> }
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
