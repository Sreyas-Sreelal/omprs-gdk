#![allow(clippy::all)]
use std::mem::transmute;

use crate::{events::EventArgs, players::Player, runtime::each_module};

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
    each_module(move |mut script| {
        script.on_player_give_damage_actor(
            Player::new(*(*(*args).list).player),
            Actor::new(*(*(*args).list).actor),
            *(*(*args).list).amount,
            *(*(*args).list).weapon,
            transmute(*(*(*args).list).part),
        );
        None
    });
}

#[repr(C)]
pub struct OnActorStreamInArgs {
    actor: *const *const std::ffi::c_void,
    forPlayer: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnActorStreamIn(args: *const EventArgs<OnActorStreamInArgs>) {
    each_module(move |mut script| {
        script.on_actor_stream_in(
            Actor::new(*(*(*args).list).actor),
            Player::new(*(*(*args).list).forPlayer),
        );
        None
    });
}

#[repr(C)]
pub struct OnActorStreamOutArgs {
    actor: *const *const std::ffi::c_void,
    forPlayer: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnActorStreamOut(args: *const EventArgs<OnActorStreamOutArgs>) {
    each_module(move |mut script| {
        script.on_actor_stream_out(
            Actor::new(*(*(*args).list).actor),
            Player::new(*(*(*args).list).forPlayer),
        );
        None
    });
}
