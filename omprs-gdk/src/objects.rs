use std::ffi::c_void;

pub struct Object {
    handle: *const c_void,
}

impl Object {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }
}

pub struct PlayerObject {
    handle: *const c_void,
}

impl PlayerObject {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }
}
