#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player, runtime::get_scripts};

#[repr(C)]
pub struct OnPlayerRequestClassArgs {
    player: *const *const std::ffi::c_void,
    classId: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerRequestClass(
    args: *const EventArgs<OnPlayerRequestClassArgs>,
) -> bool {
    let mut ret = false;
    for script in get_scripts() {
        ret = script.on_player_request_class(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).classId,
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}
