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

#[cfg(test)]
mod test {
    use std::{
        cell::RefCell,
        ops::Deref,
        rc::Rc,
        sync::{Arc, Mutex},
    };

    use tokio::spawn;

    use crate::Event;

    #[test]
    fn event() {
        let event = Event::<u32>::default();
        let summ = Rc::new(RefCell::new(0));

        let check = summ.clone();

        event.val(move |val| {
            *summ.borrow_mut() += val;
        });

        assert_eq!(*check.borrow(), 0);
        event.trigger(20);
        assert_eq!(*check.borrow(), 20);
        event.trigger(20);
        assert_eq!(*check.borrow(), 40);

        event.remove_subscribers();
        event.trigger(20);
        assert_eq!(*check.borrow(), 40);
    }

    #[test]
    fn event_once() {
        let event = Event::<u32>::default();
        let summ = Rc::new(RefCell::new(0));

        let check = summ.clone();

        event.once(move |val| {
            *summ.borrow_mut() += val;
        });

        assert_eq!(*check.borrow(), 0);
        event.trigger(20);
        assert_eq!(*check.borrow(), 20);
        event.trigger(20);
        assert_eq!(*check.borrow(), 20);
    }

    #[tokio::test]
    async fn event_once_async() {
        let event = Event::<u32>::default();
        let summ = Arc::new(Mutex::new(0));

        let recv = event.once_async();

        let res_summ = summ.clone();
        let join = spawn(async move {
            assert_eq!(summ.lock().unwrap().deref(), &0);

            let val = recv.await.unwrap();

            assert_eq!(val, 10);

            *summ.lock().unwrap() += val;
        });

        event.trigger(10);

        join.await.unwrap();

        assert_eq!(*res_summ.lock().unwrap(), 10);
    }

    #[test]
    #[should_panic(expected = "Event already has a subscriber")]
    fn double_subscriber() {
        let event: Event = Event::default();
        event.sub(|| {});
        event.sub(|| {});
    }

    #[test]
    fn debug() {
        assert_eq!("Event<i32>", &format!("{:?}", Event::<i32>::default()));
    }
}
