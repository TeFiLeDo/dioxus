use std::collections::HashMap;

pub trait RouteContent<T> {
    fn route(&self) -> T;
}

pub trait RouteUrl {
    fn from_url(segments: &[&str], parameters: HashMap<String, String>) -> Self;

    fn to_url(&self, parameters: &mut HashMap<String, String>) -> Option<String>;
}
