use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex},
};

use log::error;
use tokio::sync::oneshot::{channel, Receiver, Sender};

type Container<T> = Arc<Mutex<Option<T>>>;

#[derive(Clone)]
pub struct Event<T: Send = ()> {
    subscriber:      Container<Box<dyn FnMut(T) + 'static + Send>>,
    once_subscriber: Container<Box<dyn FnOnce(T) + 'static + Send>>,

    once_sender: Container<Sender<T>>,
}

impl<T: 'static + Send> Event<T> {
    fn check_empty(&self) {
        if self.subscriber.lock().unwrap().is_some() {
            panic!("Event already has a subscriber");
        }
        if self.once_subscriber.lock().unwrap().is_some() {
            panic!("Event already has a once_subscriber");
        }
        if self.once_sender.lock().unwrap().is_some() {
            panic!("Event already has a once_sender");
        }
    }

    pub fn sub(&self, mut action: impl FnMut() + 'static + Send) {
        self.check_empty();
        self.subscriber.lock().unwrap().replace(Box::new(move |_| {
            action();
        }));
    }

    pub fn val(&self, action: impl FnMut(T) + 'static + Send) {
        self.check_empty();
        self.subscriber.lock().unwrap().replace(Box::new(action));
    }

    pub fn once(&self, action: impl FnOnce(T) + 'static + Send) {
        self.check_empty();
        self.once_subscriber.lock().unwrap().replace(Box::new(action));
    }

    pub fn once_async(&self) -> Receiver<T> {
        self.check_empty();
        let (s, r) = channel();
        self.once_sender.lock().unwrap().replace(s);
        r
    }

    pub fn trigger(&self, value: T) {
        let mut sub = self.subscriber.lock().unwrap();
        let mut once = self.once_subscriber.lock().unwrap();
        let mut send = self.once_sender.lock().unwrap();
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
        self.subscriber.lock().unwrap().take();
        self.once_subscriber.lock().unwrap().take();
        self.once_sender.lock().unwrap().take();
    }
}

impl<T: Send> Default for Event<T> {
    fn default() -> Self {
        Self {
            subscriber:      Default::default(),
            once_subscriber: Default::default(),
            once_sender:     Default::default(),
        }
    }
}

impl<T: Send> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event<{}>", type_name::<T>(),)
    }
}
