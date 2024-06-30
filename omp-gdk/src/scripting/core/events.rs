#![allow(clippy::all)]
use crate::{events::EventArgs, types::stringview::StringView};

#[repr(C)]
pub struct OnTickArgs {
    elapsed: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnTick(args: *const EventArgs<OnTickArgs>) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.on_tick(*(*(*args).list).elapsed);
    }
}

#[repr(C)]
pub struct OnConsoleTextArgs {
    command: *const StringView,
    parameters: *const StringView,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnConsoleText(args: *const EventArgs<OnConsoleTextArgs>) -> bool {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    let mut ret = false;
    for script in scripts.iter_mut() {
        ret = script.on_console_text(
            (*(*(*args).list).command).get_data(),
            (*(*(*args).list).parameters).get_data(),
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

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
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    let mut ret = false;
    for script in scripts.iter_mut() {
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
