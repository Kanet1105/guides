use std::collections::BTreeMap;
use std::fs;
use std::io::{stdin, Result, Read};
use std::path::PathBuf;
use std::path::Path;

pub struct FileExplorer {
    root_directory: PathBuf,
    current_directory: PathBuf,
}

impl FileExplorer {
    pub fn new() -> Self {
        Self {
            root_directory: std::env::current_dir().unwrap(),
            current_directory: std::env::current_dir().unwrap(),
        }
    }

    fn get_input(&self) -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input
    }

    /// Shows directories only.
    fn get_sub_paths(&self, cwd: &PathBuf) -> BTreeMap<String, PathBuf> {
        let mut sub_paths = BTreeMap::<String, PathBuf>::new();
        let mut count: u32 = 1;

        let sub_files = fs::read_dir().unwrap();
        for file in sub_files {
            match file {
                Err(e) => println!("{:?}", e),
                Ok(file) => {
                    println!("[{}] {:?}", count, &file.file_name());
                    sub_paths.insert(count.to_string(), file.path());
                    count += 1;
                },
            }
        }
        sub_paths
    }

    fn select(&self) {
        println!("[현재 디렉토리] {:?}", &self.current_directory);
        println!("[`] 이전 메뉴");
        let sub_paths = self.get_sub_paths();
    }

    pub fn run(&self) {
        loop {
            self.select();
        }
    }
}

#[test]
fn test_module() {
    let fe = FileExplorer::new();
    fe.run();
}