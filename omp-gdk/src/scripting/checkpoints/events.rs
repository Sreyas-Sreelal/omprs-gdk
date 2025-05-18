#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player, runtime::get_scripts};

#[repr(C)]
pub struct OnPlayerEnterCheckpointArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerEnterCheckpoint(
    args: *const EventArgs<OnPlayerEnterCheckpointArgs>,
) {
    for script in get_scripts() {
        script.on_player_enter_checkpoint(Player::new(*(*(*args).list).player));
    }
}

#[repr(C)]
pub struct OnPlayerLeaveCheckpointArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerLeaveCheckpoint(
    args: *const EventArgs<OnPlayerLeaveCheckpointArgs>,
) {
    for script in get_scripts() {
        script.on_player_leave_checkpoint(Player::new(*(*(*args).list).player));
    }
}

#[repr(C)]
pub struct OnPlayerEnterRaceCheckpointArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerEnterRaceCheckpoint(
    args: *const EventArgs<OnPlayerEnterRaceCheckpointArgs>,
) {
    for script in get_scripts() {
        script.on_player_enter_race_checkpoint(Player::new(*(*(*args).list).player));
    }
}

#[repr(C)]
pub struct OnPlayerLeaveRaceCheckpointArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerLeaveRaceCheckpoint(
    args: *const EventArgs<OnPlayerLeaveRaceCheckpointArgs>,
) {
    for script in get_scripts() {
        script.on_player_leave_race_checkpoint(Player::new(*(*(*args).list).player));
    }
}
