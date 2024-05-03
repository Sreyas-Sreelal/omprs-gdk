use omprs_gdk::Events;

pub static mut RUNTIME: Option<Box<dyn Events + 'static>> = None;

#[macro_export]
macro_rules! register {
    ($name:expr) => {
        unsafe {
            omprs::RUNTIME = Some(Box::new($name));
        }
    };
}
