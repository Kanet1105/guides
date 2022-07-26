use std::cell::RefCell;

#[derive(Clone, Debug, PartialEq)]
pub struct UserState {
    logged_in: RefCell<bool>,
}

impl Default for UserState {
    fn default() -> Self {
        Self { logged_in: RefCell::new(false) }
    }
}

impl UserState {
    pub fn is_signed_in(&self) -> bool {
        let status = self.logged_in.borrow();
        status.clone()
    }
}