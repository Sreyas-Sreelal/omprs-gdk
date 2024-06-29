#![allow(clippy::all)]
use crate::events::EventArgs;

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
    command: *const std::ffi::c_char,
    parameters: *const std::ffi::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnConsoleText(args: *const EventArgs<OnConsoleTextArgs>) -> bool {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    let mut ret = false;
    for script in scripts.iter_mut() {
        ret = script.on_console_text(
            (*(*(*args).list).command).to_string(),
            (*(*(*args).list).parameters).to_string(),
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
    address: *const std::ffi::c_char,
    password: *const std::ffi::c_char,
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
            (*(*(*args).list).address).to_string(),
            (*(*(*args).list).password).to_string(),
            *(*(*args).list).success,
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}
