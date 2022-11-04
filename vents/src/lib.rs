pub mod event;
pub mod property;

pub use event::*;
pub use property::*;

#[cfg(test)]
mod test {
    use crate::{Event, Property};
    use refs::{set_current_thread_as_main, Own};
    use std::ops::Deref;

    #[test]
    fn property() {
        let mut prop = Property::new(5);
        assert_eq!(prop.deref(), &5);
        prop.on_set.sub(|val| {
            assert_eq!(val, 10);
        });
        prop.set(10);
        dbg!(prop);
    }

    #[test]
    fn event() {
        let event = Event::<u32>::default();
        event.sub(|val| {
            assert_eq!(val, 22);
        });
        event.trigger(22);
        dbg!(event);
    }

    #[test]
    fn event_set() {
        set_current_thread_as_main();
        let event = Event::<u32>::default();
        let own = Own::new(5);
        event.set(own.deref(), |rf, val| {
            assert_eq!(rf.deref(), &5);
            assert_eq!(val, 27);
        });
        event.trigger(27);
    }
}
