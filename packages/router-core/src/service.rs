use std::{
    marker::PhantomData,
    sync::{Arc, Weak},
};

use async_lock::{RwLock, RwLockWriteGuard};
use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};

use crate::{history::RoutingHistory, RouteContent, RouteUrl};

pub enum RoutingMessage<T, C: RouteContent<T>, I> {
    Subscribe(Arc<I>),
    Push(C),
    Replace(C),
    GoBack,
    GoForward,
    /// **DO NOT USE!!** Will panic in debug and be ignored in release.
    Phantom(PhantomData<T>),
}

pub struct RoutingService<T, C: RouteContent<T> + RouteUrl, I: Clone + PartialEq> {
    history: Box<dyn RoutingHistory<T, C>>,
    state: Arc<RwLock<C>>,

    receiver: UnboundedReceiver<RoutingMessage<T, C, I>>,

    subscribers: Vec<Weak<I>>,
    subscriber_updater: Box<dyn Fn(I)>,
}

impl<T, C: RouteContent<T> + RouteUrl, I: Clone + PartialEq> RoutingService<T, C, I> {
    pub fn new(
        history: Box<dyn RoutingHistory<T, C>>,
        subscriber_updater: Box<dyn Fn(I)>,
    ) -> (
        Self,
        UnboundedSender<RoutingMessage<T, C, I>>,
        Arc<RwLock<C>>,
    ) {
        let (sender, receiver) = unbounded();

        let state = Arc::new(RwLock::new(history.current()));

        (
            Self {
                history,
                state: state.clone(),
                receiver,
                subscribers: Vec::new(),
                subscriber_updater,
            },
            sender,
            state,
        )
    }

    pub fn run_until_empty(&mut self) {
        while let Ok(Some(msg)) = self.receiver.try_next() {
            match msg {
                RoutingMessage::Subscribe(subscriber) => self.subscribe(subscriber),
                RoutingMessage::Push(s) => self.history.push(s),
                RoutingMessage::Replace(s) => self.history.replace(s),
                RoutingMessage::GoBack => self.history.go_back(),
                RoutingMessage::GoForward => self.history.go_forward(),
                RoutingMessage::Phantom(_) => {
                    #[cfg(debug_assertions)]
                    panic!("received phantom routing message");
                }
            }
        }

        let mut state = Self::write_loop(&self.state);
        *state = self.history.current();
        drop(state);

        self.update_subscribers();
    }

    fn write_loop(lock: &RwLock<C>) -> RwLockWriteGuard<C> {
        loop {
            if let Some(x) = lock.try_write() {
                return x;
            }
        }
    }

    fn subscribe(&mut self, subscriber: Arc<I>) {
        self.subscribers.push(Arc::downgrade(&subscriber));
        (self.subscriber_updater)(subscriber.as_ref().clone());
    }

    fn update_subscribers(&mut self) {
        let mut encountered = Vec::new();

        self.subscribers.retain(|s| {
            s.upgrade()
                .map(|s| {
                    (!encountered.contains(s.as_ref()))
                        .then(|| {
                            let id = s.as_ref().clone();
                            encountered.push(id.clone());
                            (self.subscriber_updater)(id);
                            true
                        })
                        .unwrap_or(false)
                })
                .unwrap_or(false)
        });
    }
}
