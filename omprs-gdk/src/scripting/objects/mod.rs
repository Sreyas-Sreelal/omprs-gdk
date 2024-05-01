pub mod events;
pub mod functions;

use std::ffi::c_void;

use crate::vector::Vector3;

pub use functions::load_functions;

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

#[repr(C)]
pub struct ObjectMoveData {
    targetPos: Vector3,
    targetRot: Vector3,
    speed: f32,
}

#[repr(C)]
pub enum ObjectAttachmentType {
    None,
    Vehicle,
    Object,
    Player,
}

#[repr(C)]
pub struct ObjectAttachmentData {
    attachment_type: ObjectAttachmentType,
    syncRotation: bool,
    ID: isize,
    offset: Vector3,
    rotation: Vector3,
}

#[repr(C)]
pub enum ObjectMaterialTextAlign {
    Left,
    Center,
    Right,
}

#[repr(C)]
pub enum ObjectMaterialSize {
    Size32x32 = 10,
    Size64x32 = 20,
    Size64x64 = 30,
    Size128x32 = 40,
    Size128x64 = 50,
    Size128x128 = 60,
    Size256x32 = 70,
    Size256x64 = 80,
    Size256x128 = 90,
    Size256x256 = 100,
    Size512x64 = 110,
    Size512x128 = 120,
    Size512x256 = 130,
    Size512x512 = 140,
}
