use std::net::{TcpListener, TcpStream};
use crate::ThreadPool;

pub struct Server {
    listener: TcpListener,
    buffer_size: usize,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Self {
            listener: TcpListener::bind(address).unwrap(),
            buffer_size: 8192,
        }
    }

    pub fn run(&self) {
        let pool = ThreadPool::new(4).unwrap();

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(move || {
                println!("{:?}", stream);
            });
        }
    }

    pub fn handle_request(&self, stream: TcpStream) {
        println!("{:?}", stream);
    }
}