use crate::events::Events;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

type Script = dyn Events + 'static;
type OMPRSModule = Rc<RefCell<Script>>;

/// Runtime global object that implements all the callbacks and gamemode data
pub static mut Runtime: Vec<Box<OMPRSModule>> = Vec::new();

#[doc(hidden)]
pub static mut __terminate_event_chain: bool = false;

pub struct Scripts<'a> {
    iter: std::slice::Iter<'a, Box<OMPRSModule>>,
}

impl<'a> Iterator for Scripts<'a> {
    type Item = RefMut<'a, Script>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|script| script.borrow_mut())
    }
}

pub fn get_scripts<'a>() -> Scripts<'a> {
    let scripts = unsafe { (&raw mut crate::runtime::Runtime).as_mut().unwrap() };
    Scripts {
        iter: scripts.iter(),
    }
}
