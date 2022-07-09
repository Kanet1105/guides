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

    /// 노드 여러 개 추가 시 사용.
    pub fn register_from(&self, route_list: Vec<(String, default::Callback)>) {
        let mut router = self.router.borrow_mut();
        for (route, callback) in route_list {
            router.insert(route, callback);
        }
    }

    pub fn append_callback(&self, locator: String) {
        let call_stack = self.call_stack.borrow_mut();
        call_stack.push(locator);
    }

    fn pop_call_stack(&self) {
        let call_stack = self.call_stack.borrow();
    }

    pub fn run(&self, app: default::App, root_node: String) {
        let call_stack = self.call_stack.borrow();
        loop {
            match call_stack.is_empty() {
                true => self.append_callback("/".to_string()),
                false => {},
            }
        }
    }
}