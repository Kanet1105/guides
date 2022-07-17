use std::io::Write;
use std::sync::Arc;
use std::net::TcpListener;
use std::collections::HashMap;

use crate::build_response;
use crate::parse_request;
use crate::ThreadPool;

use super::response::page_404;

pub struct Server {
    listener: TcpListener,
    urls: HashMap<String, String>,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Self {
            listener: TcpListener::bind(address).unwrap(),
            urls: HashMap::new(),
        }
    }

    pub fn register_url(&mut self, uri: &'static str, html_path: &'static str) {
        self.urls.insert(uri.to_string(), html_path.to_string());
    }

    pub fn run(&self) -> Result<(), String> {
        let pool = ThreadPool::new(4).unwrap();
        let urls = Arc::new(self.urls.clone());

        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let urls = Arc::clone(&urls);

            pool.execute(move || {
                let request = parse_request(&mut stream);
                let url = request.get_url();
                let response = match urls.get(&url) {
                    Some(html_path) => build_response(html_path),
                    None => page_404(),
                };
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            });
        }

        Ok(())
    }
}