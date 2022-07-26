use std::rc::Rc;

use crate::components::{get_login_url, UserState};

use yew::prelude::*;

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let user_state = use_context::<Rc<UserState>>().unwrap();
    let status = user_state.is_signed_in();
    let mut login_url = String::from("0;url=");
    login_url.push_str(get_login_url().as_str());

    match status {
        true => html! { "Logged In" },
        false => html! {
            <meta http-equiv="refresh" content={ login_url } />
        },
    }
}