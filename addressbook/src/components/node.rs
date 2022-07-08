use std::collections::BTreeMap;
use std::fmt;
use std::io::stdin;

pub trait Node: fmt::Display {
    fn run(&self);
}

/// 하위 노드를 가지는 노드.
pub struct Internal {
    name: String,
    sub_node: BTreeMap<String, Box<dyn Node>>,
}

impl fmt::Display for Internal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Node for Internal {
    fn run(&self) {
        match self.sub_node.is_empty() {
            true => println!("[Warning] {} is an empty internal node.", self),
            false => self.select(),
        }
    }
}

impl Internal {
    /// 빈 트리로 초기화
    pub fn new(name: String) -> Box<Self> {
        Box::new(Self {
            name,
            sub_node: BTreeMap::new(),
        })
    }

    /// Vec<(노드 이름, 노드 타입)> 으로부터 하위 노드를 갖도록 초기화
    pub fn build(name: String, sub_node: Vec<(String, Box<dyn Node>)>) -> Box<Self> {
        Box::new(Self {
            name,
            sub_node: sub_node.into_iter().collect(),
        })
    }

    /// 초기화 후 하위 노드 추가
    pub fn add_menu(&mut self, sub_node: Vec<(String, Box<dyn Node>)>) {
        match sub_node.is_empty() {
            true => println!("[Warning] 'parameter: sub_node' is empty!"),
            false => self.sub_node = sub_node.into_iter().collect(),
        }
    }

    fn print(&self) {
        for (command_text, node) in &self.sub_node {
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
    pub fn new(name: String) -> Box<Self> {
        Box::new(Self {
            name,
        })
    }
}