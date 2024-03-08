use std::ffi::{c_char, CStr};

pub static mut OMPRS_SendClientMessage: Option<unsafe extern "C" fn(isize, isize, *const c_char)> =
    None;

pub static mut OMPRS_GetPlayerName: Option<unsafe extern "C" fn(isize, *mut c_char) -> isize> =
    None;

pub fn SendClientMessage(playerid: isize, colour: isize, message: &str) -> isize {
    unsafe { OMPRS_SendClientMessage.unwrap()(playerid, colour, cstr!(message)) }
    1
}

pub fn GetPlayerName(playerid: isize, name: &mut String) -> isize {
    let mut addr = vec![0 as c_char; 25];
    let length = unsafe { OMPRS_GetPlayerName.unwrap()(playerid, addr.as_mut_ptr()) };
    *name = from_cstr!(addr);
    length
}
