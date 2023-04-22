use crate::RouteContent;

mod memory;
pub use memory::*;

pub trait RoutingHistory<T, C: RouteContent<T>> {
    #[must_use]
    fn current(&self) -> C;

    fn push(&mut self, state: C);
    fn replace(&mut self, state: C);

    #[must_use]
    fn can_go_back(&self) -> bool;
    fn go_back(&mut self);

    #[must_use]
    fn can_go_forward(&self) -> bool;
    fn go_forward(&mut self);
}
