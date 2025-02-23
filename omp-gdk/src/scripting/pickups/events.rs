#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player};

use super::Pickup;

#[repr(C)]
pub struct OnPlayerPickUpPickupArgs {
    player: *const *const std::ffi::c_void,
    pickup: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerPickUpPickup(
    args: *const EventArgs<OnPlayerPickUpPickupArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime).as_mut().unwrap().as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.borrow_mut().on_player_pick_up_pickup(
            Player::new(*(*(*args).list).player),
            Pickup::new(*(*(*args).list).pickup),
        );
    }
}
