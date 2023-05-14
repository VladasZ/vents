use std::{fmt::Debug, ops::Deref};

use crate::Event;

#[derive(Default)]
pub struct Property<T: Send> {
    data:       T,
    pub on_set: Event<T>,
}

impl<T: 'static + Send + Clone> Property<T> {
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

impl<T: Send> Deref for Property<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T: Send + Debug> Debug for Property<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
