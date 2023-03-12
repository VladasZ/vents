use refs::{ToWeak, Weak};
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
};

pub struct Event<T = ()> {
    #[allow(clippy::type_complexity)]
    subscriber: RefCell<Option<Box<dyn FnMut(T) + 'static>>>,
    #[allow(clippy::type_complexity)]
    once_subscriber: RefCell<Option<Box<dyn FnOnce(T) + 'static>>>,
}

impl<T: 'static> Event<T> {
    pub fn sub(&self, mut action: impl FnMut() + 'static) {
        self.subscriber.replace(Some(Box::new(move |_| {
            action();
        })));
    }

    pub fn val(&self, action: impl FnMut(T) + 'static) {
        self.subscriber.replace(Some(Box::new(action)));
    }

    pub fn once(&self, action: impl FnOnce(T) + 'static) {
        self.once_subscriber.replace(Some(Box::new(action)));
    }

    pub fn set<Obj: 'static>(&self, obj: &Obj, mut action: impl FnMut(Weak<Obj>, T) + 'static) {
        let weak = obj.weak();
        self.subscriber.replace(Some(Box::new(move |value| {
            action(weak, value);
        })));
    }

    pub fn trigger(&self, value: T) {
        let mut sub = self.subscriber.borrow_mut();
        let mut once = self.once_subscriber.borrow_mut();
        if let Some(sub) = sub.as_mut() {
            (sub)(value)
        } else if let Some(sub) = once.take() {
            (sub)(value)
        }
    }

    pub fn remove_subscribers(&self) {
        self.subscriber.replace(Default::default());
        self.once_subscriber.replace(Default::default());
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self {
            subscriber: Default::default(),
            once_subscriber: Default::default(),
        }
    }
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event<{}>", std::any::type_name::<T>(),)
    }
}
