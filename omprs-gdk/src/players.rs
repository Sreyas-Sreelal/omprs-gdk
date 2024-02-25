use std::ffi::c_char;

pub static mut OMPRS_SendClientMessage: Option<unsafe extern "C" fn(isize, isize, *const c_char)> =
    None;

pub fn SendClientMessage(playerid: isize, colour: isize, message: &str) -> isize {
    unsafe { OMPRS_SendClientMessage.unwrap()(playerid, colour, cstr!(message)) }
    1
}
