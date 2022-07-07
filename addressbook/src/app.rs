use crate::components::contact::{Contact, Profile};
use crate::components::menu::{Node, Internal, Leaf};
use std::boxed::Box;

enum Event {
    New,
    Open,
    Create,
    Read,
    
}

pub fn run_app() {
    let main_menu = Internal::build("Main Menu".to_string(), vec![
        ('1'.to_string(), Box::new(Internal::new("새 주소록".to_string()))),
        ('2'.to_string(), Box::new(Internal::new("주소록 열기".to_string()))),
        ('3'.to_string(), Box::new(Internal::new("프로필 생성".to_string()))),
        ('4'.to_string(), Box::new(Internal::new("보기".to_string()))),
        ('6'.to_string(), Box::new(Internal::new("삭제".to_string()))),
        ('`'.to_string(), Box::new(Internal::new("종료".to_string()))),
    ]);

    main_menu.run();
}