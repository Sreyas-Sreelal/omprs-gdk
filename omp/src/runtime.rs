pub use omp_gdk::{Runtime, __terminate_event_chain};
/// Registers the gamemode object and it's events
#[macro_export]
macro_rules! register {
    ($name:expr) => {
        let obj = std::rc::Rc::new(std::cell::RefCell::new($name));
        omp::runtime::Runtime.with(|runtime| {
            runtime.borrow_mut().push(obj.clone());
        });
        obj
    };
}

#[macro_export]
macro_rules! terminate_event {
    ($name:expr) => {
        omp::runtime::__terminate_event_chain.with_borrow_mut(|terminate| {
            *terminate = true;
        });
        return $name
    };
}
