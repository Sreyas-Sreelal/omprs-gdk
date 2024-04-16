use std::ffi::c_void;

pub struct Vehicle {
    handle: *const c_void,
}

impl Vehicle {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }
    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }
}
