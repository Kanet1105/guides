use headless_chrome::{Browser, LaunchOptions};
use std::io::stdin;
use std::sync::Arc;
use std::mem::size_of_val;

fn command(browser: Arc<Browser>) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    if input.trim() == "1" {
        browser.new_tab().unwrap();
    }
}

fn main() {
    let mut options = LaunchOptions::default();
    options.headless = false;
    let browser = Arc::new(Browser::new(options).unwrap());
    let current_tab = browser.wait_for_initial_tab().unwrap();
    loop {
        command(browser.clone());
    }
}