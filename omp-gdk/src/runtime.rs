use crate::events::Events;

/// Runtime global object that implements all the callbacks and gamemode data
pub static mut Runtime: Option<Vec<Box<dyn Events + 'static>>> = None;

#[doc(hidden)]
pub static mut __terminate_event_chain: bool = false;
