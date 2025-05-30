#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player, runtime::each_module};

#[repr(C)]
pub struct OnPlayerEnterCheckpointArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerEnterCheckpoint(
    args: *const EventArgs<OnPlayerEnterCheckpointArgs>,
) {
    each_module(|mut script| {
        script.on_player_enter_checkpoint(Player::new(*(*(*args).list).player));
        None
    });
}

#[repr(C)]
pub struct OnPlayerLeaveCheckpointArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerLeaveCheckpoint(
    args: *const EventArgs<OnPlayerLeaveCheckpointArgs>,
) {
    each_module(|mut script| {
        script.on_player_leave_checkpoint(Player::new(*(*(*args).list).player));
        None
    });
}

#[repr(C)]
pub struct OnPlayerEnterRaceCheckpointArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerEnterRaceCheckpoint(
    args: *const EventArgs<OnPlayerEnterRaceCheckpointArgs>,
) {
    each_module(|mut script| {
        script.on_player_enter_race_checkpoint(Player::new(*(*(*args).list).player));
        None
    });
}

#[repr(C)]
pub struct OnPlayerLeaveRaceCheckpointArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerLeaveRaceCheckpoint(
    args: *const EventArgs<OnPlayerLeaveRaceCheckpointArgs>,
) {
    each_module(|mut script| {
        script.on_player_leave_race_checkpoint(Player::new(*(*(*args).list).player));
        None
    });
}
