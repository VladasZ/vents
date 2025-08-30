mod delayed_event;
mod event;
mod once_event;
mod tests;

pub use event::*;

#[cfg(feature = "tokio")]
pub mod tokio {
    pub use crate::{delayed_event::*, once_event::*};
}
