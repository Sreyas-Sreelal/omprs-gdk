use crate::events::Events;

pub static mut Runtime: Option<Box<dyn Events + 'static>> = None;
