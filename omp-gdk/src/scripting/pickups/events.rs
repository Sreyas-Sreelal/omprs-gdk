#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player, runtime::get_scripts};

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
    for mut script in get_scripts() {
        script.on_player_pick_up_pickup(
            Player::new(*(*(*args).list).player),
            Pickup::new(*(*(*args).list).pickup),
        );
    }
}
