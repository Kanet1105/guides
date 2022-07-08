use crate::core::runtime::Application;
use crate::components::node::{Internal, Leaf};

pub fn run_app() {
    let base_path = "D:\\RustProjects\\guides\\addressbook\\db";

    let main_menu = Internal::build("Main Menu".to_string(), vec![
        ('1'.to_string(), Leaf::new("새 주소록".to_string())),
        ('2'.to_string(), Internal::new("주소록 열기".to_string())),
        ('3'.to_string(), Leaf::new("프로필 생성".to_string())),
        ('4'.to_string(), Leaf::new("프로필 전체 보기".to_string())),
        ('6'.to_string(), Internal::new("프로필 찾기".to_string())),
        ('`'.to_string(), Internal::new("종료".to_string())),
    ]);

    let app = Application::new(main_menu, base_path);
    app.run();
}