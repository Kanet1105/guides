use std::collections::BTreeMap;
use std::fmt;
use std::io::stdin;

pub trait Node: fmt::Display {
    fn run(&self);
}

/// 하위 노드를 가지는 노드.
pub struct Internal {
    name: String,
    menu: BTreeMap<String, Box<dyn Node>>,
}

impl fmt::Display for Internal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Node for Internal {
    fn run(&self) {
        match self.menu.is_empty() {
            true => println!("[Warning] {} is an empty internal node.", self),
            false => self.select(),
        }
    }
}

impl Internal {
    pub fn new(name: String) -> Self {
        Self {
            name,
            menu: BTreeMap::new(),
        }
    }

    pub fn build(name: String, sub_node: Vec<(String, Box<dyn Node>)>) -> Self {
        Self {
            name,
            menu: sub_node.into_iter().collect(),
        }
    }

    fn print(&self) {
        for (command_text, node) in &self.menu {
            println!("[{}] {}", command_text, node);
        }
    }

    fn select(&self) {
        self.print();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }
}

/// 하위 노드가 없는 말단 노드.
pub struct Leaf {
    name: String,
}

impl fmt::Display for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Node for Leaf {
    fn run(&self) {
        println!("{} is a leaf node.", self);
    }
}

impl Leaf {
    pub fn new(name: String) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}