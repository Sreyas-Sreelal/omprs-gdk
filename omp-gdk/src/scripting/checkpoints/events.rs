#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player};

#[repr(C)]
pub struct OnPlayerEnterCheckpointArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerEnterCheckpoint(
    args: *const EventArgs<OnPlayerEnterCheckpointArgs>,
) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script
            .borrow_mut()
            .on_player_enter_checkpoint(Player::new(*(*(*args).list).player));
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
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script
            .borrow_mut()
            .on_player_leave_checkpoint(Player::new(*(*(*args).list).player));
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
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script
            .borrow_mut()
            .on_player_enter_race_checkpoint(Player::new(*(*(*args).list).player));
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
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script
            .borrow_mut()
            .on_player_leave_race_checkpoint(Player::new(*(*(*args).list).player));
    }
}
