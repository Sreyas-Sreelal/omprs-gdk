#![allow(clippy::all)]
use std::{mem::transmute, rc::Rc};

use crate::{events::EventArgs, players::Player};

use super::Actor;

#[repr(C)]
pub struct OnPlayerGiveDamageActorArgs {
    player: *const *const std::ffi::c_void,
    actor: *const *const std::ffi::c_void,
    amount: *const f32,
    weapon: *const i32,
    part: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerGiveDamageActor(
    args: *const EventArgs<OnPlayerGiveDamageActorArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_give_damage_actor(
            Player::new(*(*(*args).list).player),
            Actor::new(*(*(*args).list).actor),
            *(*(*args).list).amount,
            *(*(*args).list).weapon,
            transmute(*(*(*args).list).part),
        );
    }
}

#[repr(C)]
pub struct OnActorStreamInArgs {
    actor: *const *const std::ffi::c_void,
    forPlayer: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnActorStreamIn(args: *const EventArgs<OnActorStreamInArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_actor_stream_in(
            Actor::new(*(*(*args).list).actor),
            Player::new(*(*(*args).list).forPlayer),
        );
    }
}

#[repr(C)]
pub struct OnActorStreamOutArgs {
    actor: *const *const std::ffi::c_void,
    forPlayer: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnActorStreamOut(args: *const EventArgs<OnActorStreamOutArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_actor_stream_out(
            Actor::new(*(*(*args).list).actor),
            Player::new(*(*(*args).list).forPlayer),
        );
    }
}
