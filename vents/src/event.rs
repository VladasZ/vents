use std::{
    any::type_name,
    cell::RefCell,
    fmt::{Debug, Formatter},
};

use log::error;
use tokio::sync::oneshot::{channel, Receiver, Sender};

pub struct Event<T = ()> {
    #[allow(clippy::type_complexity)]
    subscriber:      RefCell<Option<Box<dyn FnMut(T) + 'static>>>,
    #[allow(clippy::type_complexity)]
    once_subscriber: RefCell<Option<Box<dyn FnOnce(T) + 'static>>>,

    once_sender: RefCell<Option<Sender<T>>>,
}

impl<T: 'static> Event<T> {
    fn check_empty(&self) {
        if self.subscriber.borrow().is_some() {
            panic!("Event already has a subscriber");
        }
        if self.once_subscriber.borrow().is_some() {
            panic!("Event already has a once_subscriber");
        }
        if self.once_sender.borrow().is_some() {
            panic!("Event already has a once_sender");
        }
    }

    pub fn sub(&self, mut action: impl FnMut() + 'static) {
        self.check_empty();
        self.subscriber.replace(Some(Box::new(move |_| {
            action();
        })));
    }

    pub fn val(&self, action: impl FnMut(T) + 'static) {
        self.check_empty();
        self.subscriber.replace(Some(Box::new(action)));
    }

    pub fn once(&self, action: impl FnOnce(T) + 'static) {
        self.check_empty();
        self.once_subscriber.replace(Some(Box::new(action)));
    }

    pub fn once_async(&self) -> Receiver<T> {
        self.check_empty();
        let (s, r) = channel();
        self.once_sender.replace(s.into());
        r
    }

    pub fn trigger(&self, value: T) {
        let mut sub = self.subscriber.borrow_mut();
        let mut once = self.once_subscriber.borrow_mut();
        let mut send = self.once_sender.borrow_mut();
        if let Some(sub) = sub.as_mut() {
            (sub)(value)
        } else if let Some(sub) = once.take() {
            (sub)(value)
        } else if let Some(send) = send.take() {
            if send.send(value).is_err() {
                error!("Failed to once send Event of type: {}", type_name::<T>())
            }
        }
    }

    pub fn remove_subscribers(&self) {
        self.subscriber.replace(Default::default());
        self.once_subscriber.replace(Default::default());
        self.once_sender.replace(Default::default());
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self {
            subscriber:      Default::default(),
            once_subscriber: Default::default(),
            once_sender:     Default::default(),
        }
    }
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event<{}>", type_name::<T>(),)
    }
}
