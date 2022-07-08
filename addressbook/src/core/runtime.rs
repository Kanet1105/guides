use crate::core::default::{Node, State};
use std::cell::RefCell;
use std::vec::Vec;
use std::rc::Rc;

/// Single Thread 에서만 안전한 runtime
pub struct Application {
    node_stack: Vec<Box<dyn Node>>,
    state: Box<dyn State>,
}

impl Application {
    pub fn new(state: Box<dyn State>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            node_stack: Vec::new(),
            state,
        }))
    }

    pub fn register(&mut self, node: Box<dyn Node>) {
        self.node_stack.push(node);
    }

    pub fn run(&mut self) {
        match self.node_stack.is_empty() {
            true => panic!("Node stack is empty; is this intended? Shutting down the program.."),
            false => {
                let node = self.node_stack.pop().unwrap();
                node.run();
            },
        }
    }
}