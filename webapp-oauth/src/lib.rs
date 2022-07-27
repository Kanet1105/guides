use std::rc::Rc;

mod components;
use components::AppState;

mod pages;
use pages::SignIn;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/signin")]
    SignIn,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { "Home" },
        Route::SignIn => html! { <SignIn /> },
        Route::NotFound => html! { <h1>{ "[404] Not Found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let user_state = use_state(|| Rc::new(AppState::default()));

    html! {
        <ContextProvider<Rc<AppState>> context={Rc::clone(&user_state)}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<Rc<AppState>>>
    }
}