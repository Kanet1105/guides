use crate::core::default;
use std::io::stdin;

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input
}

/// "/" => the root node.
pub fn main_menu(app: default::App) {
    println!(
        "[1] 전체 프로필 보기\n\
        [2] 새 프로필\n\
        [3] 프로필 찾기\n\
        [4] 종료\n"
    );
    match get_input().as_str() {
        "1" => ,
        "2" => ,
        "3" => ,
        "4" => ,
        _ => ,
    }
}

/// "/view_all" => view all profiles.
pub fn view_all(app: default::App) {
    println!(
        "[1] 다음\n\
        [2] 이전 메뉴\n"
    );
    match get_input().as_str() {
        "1" => println!("none"),
        "2" => {},
    }
}

/// "/new_profile" => create a new profile.
pub fn new_profile(app: default::App) {
    let mut name = String::new();
    let mut address = String::new();
    let mut phone = String::new();
    print!("이름 : ");
    stdin().read_line(&mut name).unwrap();
    print!("주소 : ");
    stdin().read_line(&mut address).unwrap();
    print!("전화: ");
    stdin().read_line(&mut phone).unwrap();
    println!("{}\n{}\n{}\n", name, address, phone);
}

/// "/exit" => shut down the application.
pub fn exit(app: default::App) {
    std::process::exit(0);
}