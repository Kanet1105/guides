use std::fs;
use std::path::PathBuf;

pub fn build_response(html_path: &str) -> String {
    let path = PathBuf::from(html_path);
    let status_code = match path.exists() {
        true => "200",
        false => return page_404(),
    };

    let status = format!("HTTP/1.1 {} OK", status_code);
    let contents = fs::read_to_string(path).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents,
    );
    response
}

pub fn page_404() -> String {
    let status = "HTTP/1.1 404 Error";
    let contents = "Page not found..";
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents,
    );
    response
}