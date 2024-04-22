use crate::events::Events;

pub static mut Runtime: Option<Box<dyn Events + 'static>> = None;

#[macro_export]
macro_rules! register {
    ($name:ident) => {
        unsafe {
            omprs_gdk::Runtime = Some(Box::new($name));
        }
    };
}