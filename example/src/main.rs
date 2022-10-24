use refs::Own;
use std::ops::Deref;
use vents::Event;

fn main() {
    let event: Event<()> = Event::default();

    let num = Own::new(5);

    event.set(num.deref(), |mut n, _| {
        *n += 1;
        dbg!(n);
    });

    event.trigger(());
    event.trigger(());

    println!("Hello, world!");
}
