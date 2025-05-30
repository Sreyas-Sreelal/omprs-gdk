#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player, runtime::each_module};

use super::{PlayerTextDraw, TextDraw};

#[repr(C)]
pub struct OnPlayerCancelTextDrawSelectionArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerCancelTextDrawSelection(
    args: *const EventArgs<OnPlayerCancelTextDrawSelectionArgs>,
) {
    each_module(|mut script| {
        script.on_player_cancel_text_draw_selection(Player::new(*(*(*args).list).player));
        None
    });
}

#[repr(C)]
pub struct OnPlayerCancelPlayerTextDrawSelectionArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerCancelPlayerTextDrawSelection(
    args: *const EventArgs<OnPlayerCancelPlayerTextDrawSelectionArgs>,
) {
    each_module(|mut script| {
        script.on_player_cancel_player_text_draw_selection(Player::new(*(*(*args).list).player));
        None
    });
}

#[repr(C)]
pub struct OnPlayerClickTextDrawArgs {
    player: *const *const std::ffi::c_void,
    textdraw: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerClickTextDraw(
    args: *const EventArgs<OnPlayerClickTextDrawArgs>,
) {
    each_module(|mut script| {
        script.on_player_click_text_draw(
            Player::new(*(*(*args).list).player),
            TextDraw::new(*(*(*args).list).textdraw),
        );
        None
    });
}

#[repr(C)]
pub struct OnPlayerClickPlayerTextDrawArgs {
    player: *const *const std::ffi::c_void,
    textdraw: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerClickPlayerTextDraw(
    args: *const EventArgs<OnPlayerClickPlayerTextDrawArgs>,
) {
    each_module(|mut script| {
        script.on_player_click_player_text_draw(
            Player::new(*(*(*args).list).player),
            PlayerTextDraw::new(
                *(*(*args).list).textdraw,
                Player::new(*(*(*args).list).player),
            ),
        );
        None
    });
}
