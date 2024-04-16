use crate::types::vector::Vector3;
use std::ffi::c_void;

pub struct Actor {
    handle: *const c_void,
}

impl Actor {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ActorSpawnData {
    position: Vector3,
    facingAngle: f32,
    skin: isize,
}
