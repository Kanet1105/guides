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
    match get_input().trim() {
        "1" => app.append_node("/view_all"),
        "2" => app.append_node("/new_profile"),
        "3" => app.append_node("/find_profile"),
        "4" => app.append_node("/exit"),
        _ => println!("없는 메뉴입니다. 다시 시도하세요."),
    }
}

/// "/view_all" => view all profiles.
pub fn view_all(app: default::App) {
    println!(
        "[1] 다음\n\
        [2] 이전 메뉴\n"
    );
    let input = get_input().trim();
    // match get_input().as_str() {
    //     "1" => println!("none"),
    //     "2" => {},
    // }
}

/// "/new_profile" => create a new profile.
pub fn new_profile(app: default::App) {
    let mut name = String::new();
    let mut address = String::new();
    let mut phone = String::new();
    println!("이름 : ");
    stdin().read_line(&mut name).unwrap();
    println!("주소 : ");
    stdin().read_line(&mut address).unwrap();
    println!("전화: ");
    stdin().read_line(&mut phone).unwrap();
    println!("{}\n{}\n{}\n", name, address, phone);
}

/// "/find_profile" => find a new profile based on the name.
pub fn find_profile(app: default::App) {
    let mut name = String::new();
    println!("이름으로 찾기 : ");
    stdin().read_line(&mut name).unwrap();
    println!("{}", name);
}

/// "/exit" => shut down the application.
pub fn exit(app: default::App) {
    println!("종료합니다..");
    std::process::exit(0);
}