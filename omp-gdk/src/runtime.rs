use crate::events::Events;
use std::{cell::RefCell, rc::Rc};

type OMPRSModule = Rc<RefCell<dyn Events + 'static>>;

/// Runtime global object that implements all the callbacks and gamemode data
pub static mut Runtime: Option<Vec<Box<OMPRSModule>>> = None;

#[doc(hidden)]
pub static mut __terminate_event_chain: bool = false;

pub struct Scripts<'a> {
    iter: std::slice::Iter<'a, Box<OMPRSModule>>,
}

impl<'a> Iterator for Scripts<'a> {
    type Item = &'a mut (dyn Events + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|script| unsafe { &mut *(*Rc::as_ptr(script)).as_ptr() })
    }
}

pub fn get_scripts() -> Scripts<'static> {
    let scripts = unsafe { (&raw mut crate::runtime::Runtime).as_mut() }
        .unwrap()
        .as_mut()
        .unwrap();
    Scripts {
        iter: scripts.iter(),
    }
}
