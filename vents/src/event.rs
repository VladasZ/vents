use std::{
    any::type_name,
    cell::RefCell,
    fmt::{Debug, Formatter},
};

type Callback<T> = Box<dyn FnMut(T) + 'static>;

pub struct Event<T = ()> {
    subscriber: RefCell<Option<Callback<T>>>,
}

impl<T: 'static> Event<T> {
    fn check_empty(&self) {
        assert!(
            self.subscriber.borrow().is_none(),
            "Event already has a subscriber"
        );
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
    use std::{cell::RefCell, rc::Rc};

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
