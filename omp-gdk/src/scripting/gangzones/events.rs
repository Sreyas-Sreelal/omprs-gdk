#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player};

use super::GangZone;

#[repr(C)]
pub struct OnPlayerEnterGangZoneArgs {
    player: *const *const std::ffi::c_void,
    zone: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerEnterGangZone(
    args: *const EventArgs<OnPlayerEnterGangZoneArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime).as_mut().unwrap().as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.borrow_mut().on_player_enter_gang_zone(
            Player::new(*(*(*args).list).player),
            GangZone::new(*(*(*args).list).zone),
        );
    }
}

#[repr(C)]
pub struct OnPlayerLeaveGangZoneArgs {
    player: *const *const std::ffi::c_void,
    zone: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerLeaveGangZone(
    args: *const EventArgs<OnPlayerLeaveGangZoneArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime).as_mut().unwrap().as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.borrow_mut().on_player_leave_gang_zone(
            Player::new(*(*(*args).list).player),
            GangZone::new(*(*(*args).list).zone),
        );
    }
}

#[repr(C)]
pub struct OnPlayerClickGangZoneArgs {
    player: *const *const std::ffi::c_void,
    zone: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerClickGangZone(
    args: *const EventArgs<OnPlayerClickGangZoneArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime).as_mut().unwrap().as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.borrow_mut().on_player_click_gang_zone(
            Player::new(*(*(*args).list).player),
            GangZone::new(*(*(*args).list).zone),
        );
    }
}
