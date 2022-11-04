use std::fmt::Debug;
use std::ops::Deref;

use crate::Event;

#[derive(Default)]
pub struct Property<T> {
    data: T,
    pub on_set: Event<T>,
}

impl<T: 'static + Clone> Property<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            on_set: Default::default(),
        }
    }

    pub fn set(&mut self, value: impl Into<T>) {
        self.data = value.into();
        self.on_set.trigger(self.data.clone());
    }
}

impl<T> Deref for Property<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T: Debug> Debug for Property<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
