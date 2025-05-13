#![allow(clippy::all)]
use std::{mem::transmute, rc::Rc};

use crate::{events::EventArgs, players::Player, types::vector::Vector3};

use super::{Object, ObjectAttachmentSlotData, PlayerObject};

#[repr(C)]
pub struct OnObjectMoveArgs {
    object: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnObjectMove(args: *const EventArgs<OnObjectMoveArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_object_moved(Object::new(*(*(*args).list).object));
    }
}

#[repr(C)]
pub struct OnPlayerObjectMoveArgs {
    player: *const *const std::ffi::c_void,
    object: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerObjectMove(args: *const EventArgs<OnPlayerObjectMoveArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_object_moved(
            Player::new(*(*(*args).list).player),
            PlayerObject::new(
                *(*(*args).list).object,
                Player::new(*(*(*args).list).player),
            ),
        );
    }
}

#[repr(C)]
pub struct OnPlayerEditObjectArgs {
    player: *const *const std::ffi::c_void,
    object: *const *const std::ffi::c_void,
    response: *const i32,
    offsetX: *const f32,
    offsetY: *const f32,
    offsetZ: *const f32,
    rotationX: *const f32,
    rotationY: *const f32,
    rotationZ: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerEditObject(args: *const EventArgs<OnPlayerEditObjectArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_edit_object(
            Player::new(*(*(*args).list).player),
            Object::new(*(*(*args).list).object),
            transmute(*(*(*args).list).response),
            Vector3::new(
                *(*(*args).list).offsetX,
                *(*(*args).list).offsetY,
                *(*(*args).list).offsetZ,
            ),
            Vector3::new(
                *(*(*args).list).rotationX,
                *(*(*args).list).rotationY,
                *(*(*args).list).rotationZ,
            ),
        );
    }
}

#[repr(C)]
pub struct OnPlayerEditPlayerObjectArgs {
    player: *const *const std::ffi::c_void,
    object: *const *const std::ffi::c_void,
    response: *const i32,
    offsetX: *const f32,
    offsetY: *const f32,
    offsetZ: *const f32,
    rotationX: *const f32,
    rotationY: *const f32,
    rotationZ: *const f32,
}

#[repr(C)]
pub struct OnPlayerEditAttachedObjectArgs {
    player: *const *const std::ffi::c_void,
    saved: *const bool,
    index: *const i32,
    model: *const i32,
    bone: *const i32,
    offsetX: *const f32,
    offsetY: *const f32,
    offsetZ: *const f32,
    rotationX: *const f32,
    rotationY: *const f32,
    rotationZ: *const f32,
    scaleX: *const f32,
    scaleY: *const f32,
    scaleZ: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerEditAttachedObject(
    args: *const EventArgs<OnPlayerEditAttachedObjectArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_edit_attached_object(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).index,
            *(*(*args).list).saved,
            ObjectAttachmentSlotData {
                model: *(*(*args).list).model,
                bone: *(*(*args).list).bone,
                offset: Vector3::new(
                    *(*(*args).list).offsetX,
                    *(*(*args).list).offsetY,
                    *(*(*args).list).offsetZ,
                ),
                rotation: Vector3::new(
                    *(*(*args).list).rotationX,
                    *(*(*args).list).rotationY,
                    *(*(*args).list).rotationZ,
                ),
                scale: Vector3::new(
                    *(*(*args).list).scaleX,
                    *(*(*args).list).scaleY,
                    *(*(*args).list).scaleZ,
                ),
                ..Default::default()
            },
        );
    }
}

#[repr(C)]
pub struct OnPlayerSelectObjectArgs {
    player: *const *const std::ffi::c_void,
    object: *const *const std::ffi::c_void,
    model: *const i32,
    x: *const f32,
    y: *const f32,
    z: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerSelectObject(
    args: *const EventArgs<OnPlayerSelectObjectArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_select_object(
            Player::new(*(*(*args).list).player),
            Object::new(*(*(*args).list).object),
            *(*(*args).list).model,
            Vector3::new(*(*(*args).list).x, *(*(*args).list).y, *(*(*args).list).z),
        );
    }
}

#[repr(C)]
pub struct OnPlayerSelectPlayerObjectArgs {
    player: *const *const std::ffi::c_void,
    object: *const *const std::ffi::c_void,
    model: *const i32,
    x: *const f32,
    y: *const f32,
    z: *const f32,
}
