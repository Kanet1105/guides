use std::fmt;

#[derive(Debug)]
pub struct ThreadCountError;
impl fmt::Display for ThreadCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'thread_count' must be >= 0")
    }
}