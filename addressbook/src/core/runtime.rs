use std::borrow::BorrowMut;

use crate::core::default;

pub struct Application {
    call_stack: default::CallStack,
    router: default::Router,
}

impl Application {
    pub fn new() -> Self {
        Self {
            call_stack: default::new_call_stack(),
            router: default::new_router(),
        }
    }

    /// 단일 노드 추가 시 사용.
    pub fn register_node(&self, route: String, callback: default::Callback) {
        let mut router = self.router.borrow_mut();
        router.insert(route, callback);
    }

    /// Vec 타입으로 추가 시 사용.
    pub fn register_from(&self, route_list: Vec<(String, default::Callback)>) {
        let mut router = self.router.borrow_mut();
        for (route, callback) in route_list {
            router.insert(route, callback);
        }
    }

    pub fn run(&self, locator: String, app: default::App) {
        
    }
}