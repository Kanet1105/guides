use crate::core::runtime::Application;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type App = Rc<Application>;
pub fn new_app() -> App {
    Rc::new(Application::new())
}

pub type Callback = fn(app: App);

pub type CallStack = RefCell<Vec<Callback>>;
pub fn new_call_stack() -> CallStack {
    RefCell::new(Vec::new())
}

pub type Router = RefCell<HashMap<String, Callback>>;
pub fn new_router() -> Router {
    RefCell::new(HashMap::new())
}

pub trait Node: fmt::Display {
    fn run(&self);
}

pub trait State {}