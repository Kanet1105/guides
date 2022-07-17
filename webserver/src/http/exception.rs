use std::fmt;

#[derive(Debug)]
pub struct EmptyRouteError;
impl fmt::Display for EmptyRouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The number of 'GET' route must be >= 0")
    }
}