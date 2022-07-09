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

    pub fn append_node<'a>(&self, locator: &'a str) {
        let router = self.router.borrow();
        let callback = router.get(locator);
        match callback {
            Some(callback) => {
                self.call_stack.borrow_mut().push(callback.clone());
            },
            None => panic!("No callback registered under '{}'.. Shutting down..", locator),
        }
    }

    pub fn fetch_callback(&self) -> Option<default::Callback> {
        self.call_stack.borrow_mut().pop()
    }

    pub fn run(&self, app: default::App, root_node: String) {
        let root = root_node;
        loop {
            let callback = self.fetch_callback();
            match callback {
                Some(callback) => (callback)(app.clone()),
                None => self.append_node(&root),
            }
        }
    }
}