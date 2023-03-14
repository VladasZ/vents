pub mod event;
pub mod property;

pub use event::*;
pub use property::*;

#[cfg(test)]
mod test {
    use crate::{Event, Property};
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    #[test]
    fn property() {
        let mut prop = Property::new(5);
        assert_eq!(prop.deref(), &5);
        prop.on_set.val(|val| {
            assert_eq!(val, 10);
        });
        prop.set(10);
        dbg!(prop);
    }

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
}
