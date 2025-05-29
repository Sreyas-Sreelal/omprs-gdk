pub use omp_gdk::{Runtime, __terminate_event_chain};
/// Registers the gamemode object and it's events
#[macro_export]
macro_rules! register {
    ($name:expr) => {
        let obj = std::rc::Rc::new(std::cell::RefCell::new($name));
        unsafe {
            omp::runtime::Runtime.push(Box::new(obj.clone()));
        }
        obj
    };
}

#[macro_export]
macro_rules! terminate_event {
    ($name:expr) => {
        unsafe {
            omp::runtime::__terminate_event_chain = true;
        }
        return $name
    };
}
