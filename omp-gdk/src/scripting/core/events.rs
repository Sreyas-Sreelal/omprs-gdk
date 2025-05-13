#![allow(clippy::all)]
use std::rc::Rc;

use crate::{events::EventArgs, types::stringview::StringView};

#[repr(C)]
pub struct OnTickArgs {
    elapsed: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnTick(args: *const EventArgs<OnTickArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_tick(*(*(*args).list).elapsed);
    }
}

#[repr(C)]
pub struct OnConsoleTextArgs {
    command: *const StringView,
    parameters: *const StringView,
}

// #[no_mangle]
// pub unsafe extern "C" fn OMPRS_OnConsoleText(args: *const EventArgs<OnConsoleTextArgs>) -> bool {
//     let scripts = (&raw mut crate::runtime::Runtime).as_mut().unwrap().as_mut().unwrap();
//     let mut ret = false;
//     for script in scripts.iter_mut() {
//         ret = script.borrow_mut().on_console_text(
//             (*(*(*args).list).command).get_data(),
//             (*(*(*args).list).parameters).get_data(),
//         );
//         if crate::runtime::__terminate_event_chain {
//             crate::runtime::__terminate_event_chain = false;
//             return ret;
//         }
//     }
//     ret
// }

#[repr(C)]
pub struct OnRconLoginAttemptArgs {
    address: *const StringView,
    password: *const StringView,
    success: *const bool,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnRconLoginAttempt(
    args: *const EventArgs<OnRconLoginAttemptArgs>,
) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_rcon_login_attempt(
            (*(*(*args).list).address).get_data(),
            (*(*(*args).list).password).get_data(),
            *(*(*args).list).success,
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}
