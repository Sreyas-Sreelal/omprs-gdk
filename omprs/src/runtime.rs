pub use omprs_gdk::Runtime;
#[macro_export]
macro_rules! register {
    ($name:expr) => {
        unsafe {
            omprs::runtime::Runtime = Some(Box::new($name));
        }
    };
}
