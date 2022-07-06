use std::cell::RefCell;
use std::io::stdin;
use std::rc::Rc;

use crate::components::menu::{Entry, MainMenu};

enum Actions {
    NewAddress,
    OpenAddress,
    Create,
    Read,
    Update,
    Delete,
}

struct Application {
    main_menu: MainMenu<Entry>,
}

impl Application {
    fn init() -> Self {
        Self {
            main_menu: MainMenu::<Entry>::init(),
        }
    }

    /// Print the menu and wait for a user input.
    fn print_menus() {}
}

/// The entry.
pub fn run_app() {
    let app = Rc::new(Application::init());

    loop {
        
    }
}