#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player, runtime::each_module};

#[repr(C)]
pub struct OnPlayerRequestClassArgs {
    player: *const *const std::ffi::c_void,
    classId: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerRequestClass(
    args: *const EventArgs<OnPlayerRequestClassArgs>,
) -> bool {
    each_module(|mut script| {
        Some(script.on_player_request_class(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).classId,
        ))
    })
    .unwrap()
}
