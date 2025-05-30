#![allow(clippy::all)]
use std::collections::VecDeque;

use crate::{events::EventArgs, runtime::each_module, types::stringview::StringView};

#[repr(C)]
pub struct OnTickArgs {
    elapsed: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnTick(args: *const EventArgs<OnTickArgs>) {
    let api_calls: VecDeque<_> =
        crate::runtime::API_QUEUE.with_borrow_mut(|queue| std::mem::take(queue));
    // drop mutable reference to api queue before calling api callbacks
    for callback in api_calls {
        callback();
    }

    each_module(move |mut script| {
        script.on_tick(*(*(*args).list).elapsed);
        None
    });
}

#[repr(C)]
pub struct OnConsoleTextArgs {
    command: *const StringView,
    parameters: *const StringView,
}

// #[no_mangle]
// pub unsafe extern "C" fn OMPRS_OnConsoleText(args: *const EventArgs<OnConsoleTextArgs>) -> bool {
//     each_script(|mut script| {
//         Some(script.on_console_text(
//             (*(*(*args).list).command).get_data(),
//             (*(*(*args).list).parameters).get_data(),
//         ))
//     })
//     .unwrap()
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
    each_module(move |mut script| {
        Some(script.on_rcon_login_attempt(
            (*(*(*args).list).address).get_data(),
            (*(*(*args).list).password).get_data(),
            *(*(*args).list).success,
        ))
    })
    .unwrap()
}
