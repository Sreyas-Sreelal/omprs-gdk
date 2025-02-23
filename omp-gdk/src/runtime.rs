use crate::events::Events;
use std::{cell::RefCell, rc::Rc};

type OMPRSModule = Rc<RefCell<dyn Events + 'static>>;

/// Runtime global object that implements all the callbacks and gamemode data
pub static mut Runtime: Option<Vec<Box<OMPRSModule>>> = None;

#[doc(hidden)]
pub static mut __terminate_event_chain: bool = false;
