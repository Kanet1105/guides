use crate::core::default::Node;
use std::cell::RefCell;
use std::collections::vec_deque::VecDeque;
use std::path::PathBuf;
use std::rc::Rc;

/// Single Thread 에서만 안전한 runtime
pub struct Application {
    call_stack: RefCell<VecDeque<Box<dyn Node>>>,
    base_path: PathBuf,
}

impl Application {
    pub fn new(root_node: Box<dyn Node>, base_path: &'static str) -> Rc<Self> {
        let mut stack = VecDeque::new();
        stack.push_back(root_node);

        let mut path = PathBuf::from(base_path);

        Rc::new(Self {
            call_stack: RefCell::new(stack),
            base_path: path,
        })
    }

    pub fn register(&self, node: Box<dyn Node>) {
        let mut controller = self.call_stack.borrow_mut();
        controller.push_back(node);
    }

    pub fn run(&self) {
        let mut controller = self.call_stack.borrow_mut();

        match controller.is_empty() {
            true => println!("Shutting down the program.."),
            false => {
                let node = controller.pop_front().unwrap();
                node.run();
            },
        }
    }
}