#![allow(clippy::all)]
use std::rc::Rc;

use crate::{events::EventArgs, players::Player};

#[repr(C)]
pub struct OnPlayerRequestClassArgs {
    player: *const *const std::ffi::c_void,
    classId: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerRequestClass(
    args: *const EventArgs<OnPlayerRequestClassArgs>,
) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
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
