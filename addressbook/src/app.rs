use crate::core::default::{new_app, new_callback};
use crate::components::menu;

pub fn run_app() {
    let app = new_app();
    app.register_from(vec![
        ("/".to_string(), new_callback(menu::main_menu)),
        ("/new_address_book".to_string(), new_callback(menu::new_address_book)),
        ("/load_address_book".to_string(), new_callback(menu::load_address_book)),
        ("/save_address_book".to_string(), new_callback(menu::save_address_book)),
        ("/view_all".to_string(), new_callback(menu::view_all)),
        ("/new_profile".to_string(), new_callback(menu::new_profile)),
        ("/find_profile".to_string(), new_callback(menu::find_profile)),
        ("/exit".to_string(), new_callback(menu::exit)),
    ]);
    app.run(app.clone(), "/".to_string());
}