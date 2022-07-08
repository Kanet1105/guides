use crate::core::runtime::Application;
use std::cell::RefCell;
use std::io::stdin;
use std::rc::Rc;

pub fn exit(app: Rc<RefCell<Application>>) {
    println!("[`] 종료\n[_] 돌아가기\n");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    match input.as_str() {
        "1" => std::process::exit(0),
        _ => println!("continued"),
    }
}