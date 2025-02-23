#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player};

#[repr(C)]
pub struct OnPlayerSelectedMenuRowArgs {
    player: *const *const std::ffi::c_void,
    row: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerSelectedMenuRow(
    args: *const EventArgs<OnPlayerSelectedMenuRowArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter_mut() {
        script.borrow_mut().on_player_selected_menu_row(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).row,
        );
    }
}

#[repr(C)]
pub struct OnPlayerExitedMenuArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerExitedMenu(args: *const EventArgs<OnPlayerExitedMenuArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter_mut() {
        script
            .borrow_mut()
            .on_player_exited_menu(Player::new(*(*(*args).list).player));
    }
}
