pub mod event;
pub mod property;

pub use event::*;
pub use property::*;

#[cfg(test)]
mod test {
    use std::{
        cell::RefCell,
        ops::Deref,
        rc::Rc,
        sync::{Arc, Mutex},
    };

    use tokio::spawn;

    use crate::{Event, Property};

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
}
