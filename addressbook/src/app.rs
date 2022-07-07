use crate::components::menu::{Entry, Menu};

pub fn run_app() {
    let main_menu = Menu::new(vec![
        ('1'.to_string(), Entry::new("새 주소록 만들기")),
        ('2'.to_string(), Entry::new("기존 주소록 열기")),
        ('3'.to_string(), Entry::new("새 주소")),
        ('4'.to_string(), Entry::new("찾기")),
        ('5'.to_string(), Entry::new("수정")), 
        ('6'.to_string(), Entry::new("삭제")),
        ('`'.to_string(), Entry::new("종료")),
    ]);

    main_menu.run();
}