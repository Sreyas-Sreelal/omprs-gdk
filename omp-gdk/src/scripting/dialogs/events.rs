#![allow(clippy::all)]
use crate::{
    events::EventArgs, players::Player, runtime::get_scripts, types::stringview::StringView,
};
use std::mem::transmute;

#[repr(C)]
pub struct OnDialogResponseArgs {
    player: *const *const std::ffi::c_void,
    dialogId: *const i32,
    response: *const i32,
    listItem: *const i32,
    inputText: *const StringView,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnDialogResponse(args: *const EventArgs<OnDialogResponseArgs>) {
    for mut script in get_scripts() {
        script.on_dialog_response(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).dialogId,
            transmute(*(*(*args).list).response),
            *(*(*args).list).listItem,
            (*(*(*args).list).inputText).get_data(),
        );
    }
}
