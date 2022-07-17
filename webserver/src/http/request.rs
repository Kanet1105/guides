use std::fmt;
use std::io::Read;
use std::net::TcpStream;

pub struct Request {
    kind: String,
    url: String,
    protocol: String,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "[Type] {}\n[URL] {}\n[Protocol] {}\n",
            self.kind, self.url, self.protocol
        )
    }
}

impl Request {
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}

pub fn parse_request(stream: &mut TcpStream) -> Request {
    let mut buffer = [0; 8192];
    stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer);
    let (haystack, _) = message.split_once("\r\n").unwrap();
    let needle: Vec<&str> = haystack.split(" ").collect();

    Request {
        kind: needle[0].to_string(),
        url: needle[1].to_string(),
        protocol: needle[2].to_string(),
    }
}