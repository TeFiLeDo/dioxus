use std::{marker::PhantomData, mem::replace};

use crate::RouteContent;

use super::RoutingHistory;

pub struct MemoryHistory<T, C: RouteContent<T> + Clone> {
    current: C,
    past: Vec<C>,
    future: Vec<C>,
    phantom: PhantomData<T>,
}

impl<T, C: RouteContent<T> + Clone> MemoryHistory<T, C> {
    pub fn new(initial: C) -> Self {
        Self {
            current: initial,
            past: Vec::new(),
            future: Vec::new(),
            phantom: PhantomData,
        }
    }
}

impl<T, C: RouteContent<T> + Clone + Default> Default for MemoryHistory<T, C> {
    fn default() -> Self {
        Self::new(C::default())
    }
}

impl<T, C: RouteContent<T> + Clone> RoutingHistory<T, C> for MemoryHistory<T, C> {
    fn current(&self) -> C {
        self.current.clone()
    }

    fn push(&mut self, state: C) {
        self.past.push(replace(&mut self.current, state));
    }

    fn replace(&mut self, state: C) {
        self.current = state;
    }

    fn can_go_back(&self) -> bool {
        !self.past.is_empty()
    }

    fn go_back(&mut self) {
        if let Some(previous) = self.past.pop() {
            self.future.push(replace(&mut self.current, previous));
        }
    }

    fn can_go_forward(&self) -> bool {
        !self.future.is_empty()
    }

    fn go_forward(&mut self) {
        if let Some(next) = self.future.pop() {
            self.past.push(replace(&mut self.current, next));
        }
    }
}
