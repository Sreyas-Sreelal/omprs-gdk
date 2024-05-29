pub use omp_gdk::{Runtime, __terminate_event_chain};
/// Registers the gamemode object and it's events
#[macro_export]
macro_rules! register {
    ($name:expr) => {
        unsafe {
            if omp::runtime::Runtime.is_none() {
                omp::runtime::Runtime = Some(Vec::new());
            }
            omp::runtime::Runtime
                .as_mut()
                .unwrap()
                .push(Box::new($name));
        }
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
