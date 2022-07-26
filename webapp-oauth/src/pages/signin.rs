use std::rc::Rc;

use crate::components::UserState;

use yew::prelude::*;

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let user_state = use_context::<Rc<UserState>>().unwrap();
    let status = user_state.is_signed_in();
    match status {
        true => html! { "Logged In" },
        false => html! {
            <meta http-equiv="refresh" content="0;url=https://www.google.com" />
        },
    }
}