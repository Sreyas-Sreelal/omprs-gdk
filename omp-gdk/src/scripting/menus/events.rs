#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player, runtime::each_module};

#[repr(C)]
pub struct OnPlayerSelectedMenuRowArgs {
    player: *const *const std::ffi::c_void,
    row: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerSelectedMenuRow(
    args: *const EventArgs<OnPlayerSelectedMenuRowArgs>,
) {
    each_module(move |mut script| {
        script.on_player_selected_menu_row(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).row,
        );
        None
    });
}

#[repr(C)]
pub struct OnPlayerExitedMenuArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerExitedMenu(args: *const EventArgs<OnPlayerExitedMenuArgs>) {
    each_module(move |mut script| {
        script.on_player_exited_menu(Player::new(*(*(*args).list).player));
        None
    });
}
