#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player, types::stringview::StringView};
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
    let scripts = (&raw mut crate::runtime::Runtime).as_mut().unwrap().as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.borrow_mut().on_dialog_response(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).dialogId,
            transmute(*(*(*args).list).response),
            *(*(*args).list).listItem,
            (*(*(*args).list).inputText).get_data(),
        );
    }
}
