use crate::core::filesystem::FileExplorer;
use crate::core::runtime::Application;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type App = Rc<Application>;
pub fn new_app() -> App {
    Rc::new(Application::new())
}

pub type Callback = Rc<fn(app: App)>;
pub fn new_callback(function: fn(app: App)) -> Callback {
    Rc::new(function)
}

pub type CallStack = RefCell<Vec<Callback>>;
pub fn new_call_stack() -> CallStack {
    RefCell::new(Vec::new())
}

pub type Router = RefCell<HashMap<String, Callback>>;
pub fn new_router() -> Router {
    RefCell::new(HashMap::new())
}

pub type Navigator = Rc<FileExplorer>;
pub fn new_navigator() -> Navigator {
    Rc::new(FileExplorer::new())
}