pub mod http;
pub use http::server::Server;
pub use http::request::Request;
pub use http::request::parse_request;
pub use http::response::build_response;

pub mod parallel;
pub use parallel::threadpool::ThreadPool;