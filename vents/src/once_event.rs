use std::{
    any::type_name,
    cell::RefCell,
    fmt::{Debug, Formatter},
};

use log::error;
use tokio::sync::oneshot::{channel, Receiver, Sender};

type Callback<T> = Box<dyn FnOnce(T) + 'static>;

pub struct OnceEvent<T = ()> {
    once_subscriber: RefCell<Option<Callback<T>>>,
    once_sender:     RefCell<Option<Sender<T>>>,
}

impl<T: 'static> OnceEvent<T> {
    fn check_empty(&self) {
        assert!(
            self.once_sender.borrow().is_none(),
            "Event already has once_sender"
        );
        assert!(
            self.once_subscriber.borrow().is_none(),
            "Event already has once_subscriber"
        );
    }

    pub fn sub(&self, action: impl FnOnce() + 'static) {
        self.check_empty();
        self.once_subscriber.replace(Some(Box::new(|_| action())));
    }

    pub fn val(&self, action: impl FnOnce(T) + 'static) {
        self.check_empty();
        self.once_subscriber.replace(Some(Box::new(action)));
    }

    pub fn val_async(&self) -> Receiver<T> {
        self.check_empty();
        let (s, r) = channel();
        self.once_sender.replace(s.into());
        r
    }

    pub fn trigger(&self, value: T) {
        if let Some(sub) = self.once_subscriber.borrow_mut().take() {
            (sub)(value);
        } else if let Some(send) = self.once_sender.borrow_mut().take() {
            if send.send(value).is_err() {
                error!("Failed to once send OnceEvent of type: {}", type_name::<T>());
            }
        }
    }

    pub fn remove_subscribers(&self) {
        self.once_subscriber.replace(None);
        self.once_sender.replace(None);
    }
}

impl<T> Default for OnceEvent<T> {
    fn default() -> Self {
        Self {
            once_subscriber: RefCell::default(),
            once_sender:     RefCell::default(),
        }
    }
}

impl<T> Debug for OnceEvent<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OnceEvent<{}>", type_name::<T>(),)
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

    use crate::OnceEvent;

    #[test]
    fn event_once() {
        let event = OnceEvent::<u32>::default();
        let summ = Rc::new(RefCell::new(0));

        let check = summ.clone();

        let sum_2 = summ.clone();
        event.val(move |val| {
            *sum_2.borrow_mut() += val;
        });

        assert_eq!(*check.borrow(), 0);
        event.trigger(20);
        assert_eq!(*check.borrow(), 20);
        event.trigger(20);
        assert_eq!(*check.borrow(), 20);

        event.val(move |val| {
            *summ.borrow_mut() += val;
        });

        event.remove_subscribers();

        event.trigger(20);
        assert_eq!(*check.borrow(), 20);
    }

    #[tokio::test]
    async fn event_once_async() {
        let event = OnceEvent::<u32>::default();
        let summ = Arc::new(Mutex::new(0));

        let recv = event.val_async();

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
    #[should_panic(expected = "Event already has once_subscriber")]
    fn double_subscriber() {
        let event: OnceEvent = OnceEvent::default();
        event.sub(|| {});
        event.val(|_| {});
    }

    #[test]
    fn debug() {
        assert_eq!("OnceEvent<i32>", &format!("{:?}", OnceEvent::<i32>::default()));
    }
}
