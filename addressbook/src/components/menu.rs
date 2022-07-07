use std::collections::BTreeMap;
use std::fmt;
use std::io::stdin;

#[derive(Debug)]
pub struct Entry {
    display_text: &'static str,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_text)
    }
}

impl Entry {
    pub fn new(display_text: &'static str) -> Self {
        Self {
            display_text,
        }
    }
}

pub struct Menu<Entry> {
    entry: BTreeMap<String, Entry>,
}

impl fmt::Display for Menu<Entry> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut menu_table = String::new();
        for (key, value) in &self.entry {
            let row = format!("[{}] {}\n", key, value);
            menu_table.push_str(&row);
        }

        write!(f, "{}", menu_table)
    }
}

impl Menu<Entry> {
    pub fn new(entry_list: Vec<(String, Entry)>) -> Self {
        Self {
            entry: entry_list.into_iter().collect(),
        }
    }

    pub fn run(&self) {
        // display the menu and wait for a user input.
        println!("{}", self);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // fetch the callback.
        let value = self.entry.get(input.trim());
        println!("{:?}", value);
    }
}