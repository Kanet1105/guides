use std::cell::RefCell;

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    logged_in: RefCell<bool>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { logged_in: RefCell::new(false) }
    }
}

impl AppState {
    pub fn is_signed_in(&self) -> bool {
        let status = self.logged_in.borrow();
        *status
    }
}