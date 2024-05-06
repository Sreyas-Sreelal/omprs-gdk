pub use omp_gdk::Runtime;
#[macro_export]
macro_rules! register {
    ($name:expr) => {
        unsafe {
            omp::runtime::Runtime = Some(Box::new($name));
        }
    };
}
