use crate::core::default;
use std::io::stdin;

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input
}

/// "/" => The root node.
pub fn main_menu(app: default::App) {
    println!(
        "[1] 새 주소록\n\
        [2] 주소록 열기\n\
        [3] 주소록 저장\n\
        [4] 전체 프로필 보기\n\
        [5] 새 프로필\n\
        [6] 프로필 찾기\n\
        [`] 종료\n"
    );
    match get_input().trim() {
        "1" => app.append_node("/new_address_book"),
        "2" => app.append_node("/load_address_book"),
        "3" => app.append_node("/save_address_book"),
        "4" => app.append_node("/view_all"),
        "5" => app.append_node("/new_profile"),
        "6" => app.append_node("/find_profile"),
        "`" => app.append_node("/exit"),
        _ => println!("없는 메뉴입니다. 다시 시도하세요."),
    }
}

/// "/new_address_book" => Create a new address book.
pub fn new_address_book(app: default::App) {
    println!("주소록 이름: ");
    let input = get_input().trim();
}

/// "/load_address_book" => Load an address book.
pub fn load_address_book(app: default::App) {
    println!("주소록 이름: ");
    let input = get_input().trim();
}

/// "/save_address_book" => Save the current address book.
pub fn save_address_book(app: default::App) {
    
}

/// "/view_all" => View all profiles.
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

/// "/new_profile" => Create a new profile.
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

/// "/find_profile" => Find a new profile based on the name.
pub fn find_profile(app: default::App) {
    let mut name = String::new();
    println!("이름으로 찾기 : ");
    stdin().read_line(&mut name).unwrap();
    println!("{}", name);
}

/// "/exit" => Shut down the application.
pub fn exit(app: default::App) {
    println!("종료합니다..");
    std::process::exit(0);
}