use std::{
    any::type_name,
    cell::RefCell,
    fmt::{Debug, Formatter},
};

type Callback<T> = Box<dyn FnMut(T) + Send + 'static>;

pub struct Event<T = ()> {
    subscriber: RefCell<Option<Callback<T>>>,
}

impl<T: 'static> Event<T> {
    pub const fn const_default() -> Self {
        Self {
            subscriber: RefCell::new(None),
        }
    }

    fn check_empty(&self) {
        assert!(
            self.subscriber.borrow().is_none(),
            "Event already has a subscriber"
        );
    }

    pub fn sub(&self, mut action: impl FnMut() + Send + 'static) {
        self.check_empty();
        self.subscriber.replace(Some(Box::new(move |_| {
            action();
        })));
    }

    pub fn val(&self, action: impl FnMut(T) + Send + 'static) {
        self.check_empty();
        self.subscriber.replace(Some(Box::new(action)));
    }

    pub fn trigger(&self, value: T) {
        if let Some(sub) = self.subscriber.borrow_mut().as_mut() {
            (sub)(value);
        }
    }

    pub fn remove_subscribers(&self) {
        self.subscriber.replace(None);
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self {
            subscriber: RefCell::default(),
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
    use std::sync::{Arc, Mutex};

    use crate::Event;

    #[test]
    fn event() {
        let event = Event::<u32>::default();

        let summ = Arc::new(Mutex::new(0));

        let summ_2 = summ.clone();
        event.val(move |val| {
            *summ_2.lock().unwrap() += val;
        });

        assert_eq!(*summ.lock().unwrap(), 0);
        event.trigger(20);
        assert_eq!(*summ.lock().unwrap(), 20);
        event.trigger(20);
        assert_eq!(*summ.lock().unwrap(), 40);

        event.remove_subscribers();
        event.trigger(20);
        assert_eq!(*summ.lock().unwrap(), 40);
    }

    static EVENT: Mutex<Event<()>> = Mutex::new(Event::const_default());

    #[test]
    #[should_panic(expected = "Event already has a subscriber")]
    fn double_subscriber() {
        let event = EVENT.lock().unwrap();
        event.sub(|| {});
        event.sub(|| {});
    }

    #[test]
    fn debug() {
        assert_eq!("Event<i32>", &format!("{:?}", Event::<i32>::default()));
    }
}
