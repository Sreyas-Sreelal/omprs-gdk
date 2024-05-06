pub use omp_gdk::Runtime;
/// Registers the gamemode object and it's events
#[macro_export]
macro_rules! register {
    ($name:expr) => {
        unsafe {
            omp::runtime::Runtime = Some(Box::new($name));
        }
    };
}
