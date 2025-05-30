use crate::events::Events;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

type OMPRSModule = Rc<RefCell<dyn Events + 'static>>;

thread_local! {
    /// Runtime global object that implements all the callbacks and gamemode data
    pub static Runtime: RefCell<Vec<OMPRSModule>> = RefCell::new(Vec::new());

    #[doc(hidden)]
    pub static __terminate_event_chain: RefCell<bool> = RefCell::new(false);
}

pub fn each_module<F>(mut f: F) -> Option<bool>
where
    F: FnMut(RefMut<dyn Events>) -> Option<bool>,
{
    let mut result = None;

    Runtime.with(|runtime| {
        for module in runtime.borrow().iter() {
            let ret = f(module.borrow_mut());
            result = ret;
            if result.is_none() {
                continue;
            }

            let mut break_iteration = false;
            crate::runtime::__terminate_event_chain.with_borrow_mut(|terminate| {
                if *terminate {
                    *terminate = false;
                    break_iteration = true;
                }
            });
            if break_iteration {
                break;
            }
        }
    });

    result
}
