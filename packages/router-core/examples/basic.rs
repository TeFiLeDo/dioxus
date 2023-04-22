use std::collections::HashMap;

use async_lock::{RwLock, RwLockReadGuard};
use dioxus_router_core::*;

#[derive(Clone)]
enum Root {
    Index,
    Nesting(Nested),
}

impl RouteContent<String> for Root {
    fn route(&self) -> String {
        match self {
            Root::Index => "index".to_string(),
            Root::Nesting(n) => format!("nesting - {}", n.route()),
        }
    }
}

impl RouteUrl for Root {
    fn from_url(segments: &[&str], parameters: HashMap<String, String>) -> Self {
        if segments.is_empty() {
            return Self::Index;
        }

        match segments[0] {
            "nesting" => Self::Nesting(Nested::from_url(&segments[1..], parameters)),
            _ => panic!("invalid route"),
        }
    }

    fn to_url(&self, parameters: &mut HashMap<String, String>) -> Option<String> {
        match self {
            Root::Index => None,
            Root::Nesting(_) => Some("nesting".to_string()),
        }
    }
}

#[derive(Clone)]
enum Nested {
    Index,
    Static,
}

impl RouteContent<String> for Nested {
    fn route(&self) -> String {
        match self {
            Nested::Index => "index",
            Nested::Static => "static",
        }
        .to_string()
    }
}

impl RouteUrl for Nested {
    fn from_url(segments: &[&str], parameters: HashMap<String, String>) -> Self {
        if segments.is_empty() {
            return Self::Index;
        }

        match segments[0] {
            "static" => Self::Static,
            _ => panic!("invalid route"),
        }
    }

    fn to_url(&self, parameters: &mut HashMap<String, String>) -> Option<String> {
        match self {
            Nested::Index => return None,
            Nested::Static => Some("static".to_string()),
        }
    }
}

fn main() {
    let history = Box::new(MemoryHistory::new(Root::Index));
    let (mut service, sender, state) =
        RoutingService::new(history, Box::new(|i: i32| println!("updating {i}")));

    render(&state);

    service.run_until_empty();
    render(&state);

    sender
        .unbounded_send(RoutingMessage::Replace(Root::Nesting(Nested::Index)))
        .unwrap();
    service.run_until_empty();
    render(&state);

    sender
        .unbounded_send(RoutingMessage::Push(Root::Nesting(Nested::Static)))
        .unwrap();
    service.run_until_empty();
    render(&state);

    sender.unbounded_send(RoutingMessage::GoBack).unwrap();
    service.run_until_empty();
    render(&state);

    sender.unbounded_send(RoutingMessage::GoForward).unwrap();
    service.run_until_empty();
    render(&state);
}

fn render(state: &RwLock<Root>) {
    let guard = read_lock(state);
    println!("{}", (*guard).route());
}

fn read_lock<T>(lock: &RwLock<T>) -> RwLockReadGuard<T> {
    loop {
        if let Some(guard) = lock.try_read() {
            return guard;
        }
    }
}
