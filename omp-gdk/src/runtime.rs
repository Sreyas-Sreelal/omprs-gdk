use crate::events::Events;

/// Runtime global object that implements all the callbacks and gamemode data
pub static mut Runtime: Option<Box<dyn Events + 'static>> = None;
