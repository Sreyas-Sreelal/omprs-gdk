use std::{
    ffi::c_char,
    str::from_utf8,
};

pub static mut OMPRS_SendClientMessage: Option<unsafe extern "C" fn(isize, isize, *const c_char)> =
    None;

pub static mut OMPRS_GetPlayerName: Option<unsafe extern "C" fn(isize, *mut u8) -> isize> = None;

pub fn SendClientMessage(playerid: isize, colour: isize, message: &str) -> isize {
    unsafe { OMPRS_SendClientMessage.unwrap()(playerid, colour, cstr!(message)) }
    1
}

pub fn GetPlayerName(playerid: isize, name: &mut String) -> isize {
    let mut addr = vec![0u8; 25];
    let length = unsafe { OMPRS_GetPlayerName.unwrap()(playerid, addr.as_mut_ptr()) };

    // probably not the best way to do this, need more research
    let addr: Vec<u8> = addr
        .to_vec()
        .iter()
        .filter_map(|x| if *x != b'\0' { Some(*x) } else { None })
        .collect();
    *name = from_utf8(&addr).unwrap().to_owned();

    length
}
