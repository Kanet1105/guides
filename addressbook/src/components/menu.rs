use std::fmt;

trait Menu: fmt::Display {
    fn init() -> Self;
    fn display();
    fn run(&self);
}

pub struct Entry {
    command_text: &'static str,
    display_text: &'static str,
    // callback: (),
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.command_text, self.display_text)
    }
}

impl Entry {
    fn build(command_text: &'static str, display_text: &'static str) -> Self {
        Self {
            command_text,
            display_text,
        }
    }
}

pub struct MainMenu<Entry> {
    new_address: Entry,
    open_address: Entry,
    create: Entry,
    read: Entry,
    update: Entry,
    delete: Entry,
}

impl fmt::Display for MainMenu<Entry> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}\n{}\n{}\n{}\n",
            self.new_address,
            self.open_address,
            self.create,
            self.read,
            self.update,
            self.delete,
        )
    }
}

impl MainMenu<Entry> {
    pub fn init() -> Self {
        Self {
            new_address: Entry::build("1", "새 주소록 만들기"),
            open_address: Entry::build("2", "기존 주소록 열기"),
            create: Entry::build("3", "새 주소"),
            read: Entry::build("4", "찾기"),
            update: Entry::build("5", "수정"),
            delete: Entry::build("6", "삭제"),
        }
    }

    pub fn display(&self) {
        println!("{}", self);
    }
}