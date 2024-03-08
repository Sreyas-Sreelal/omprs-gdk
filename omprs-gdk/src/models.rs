use std::ffi::{c_char, CStr};

// Address

pub static mut OMPRS_AddCharModel: Option<
    unsafe extern "C" fn(isize, isize, *const c_char, *const c_char) -> bool,
> = None;

pub static mut OMPRS_AddSimpleModel: Option<
    unsafe extern "C" fn(isize, isize, isize, *const c_char, *const c_char) -> bool,
> = None;

pub static mut OMPRS_AddSimpleModelTimed: Option<
    unsafe extern "C" fn(isize, isize, isize, *const c_char, *const c_char, isize, isize) -> bool,
> = None;

pub static mut OMPRS_GetPlayerCustomSkin: Option<unsafe extern "C" fn(isize) -> isize> = None;

pub static mut OMPRS_RedirectDownload: Option<unsafe extern "C" fn(isize, *const c_char) -> bool> =
    None;

pub static mut OMPRS_FindModelFileNameFromCRC: Option<
    unsafe extern "C" fn(isize, *mut c_char) -> bool,
> = None;

pub static mut OMPRS_FindTextureFileNameFromCRC: Option<
    unsafe extern "C" fn(isize, *mut c_char) -> bool,
> = None;

pub static mut OMPRS_IsValidCustomModel: Option<unsafe extern "C" fn(isize) -> bool> = None;

pub static mut OMPRS_GetCustomModelPath: Option<
    unsafe extern "C" fn(isize, *mut c_char, *mut c_char) -> bool,
> = None;

// Rust functions

pub fn AddCharModel(baseid: isize, newid: isize, dffname: &str, txdname: &str) -> bool {
    unsafe { OMPRS_AddCharModel.unwrap()(baseid, newid, cstr!(dffname), cstr!(txdname)) }
}

pub fn AddSimpleModel(
    virtualworld: isize,
    baseid: isize,
    newid: isize,
    dffname: &str,
    txdname: &str,
) -> bool {
    unsafe {
        OMPRS_AddSimpleModel.unwrap()(virtualworld, baseid, newid, cstr!(dffname), cstr!(txdname))
    }
}

pub fn AddSimpleModelTimed(
    virtualworld: isize,
    baseid: isize,
    newid: isize,
    dffname: &str,
    txdname: &str,
    timeon: isize,
    timeoff: isize,
) -> bool {
    unsafe {
        OMPRS_AddSimpleModelTimed.unwrap()(
            virtualworld,
            baseid,
            newid,
            cstr!(dffname),
            cstr!(txdname),
            timeon,
            timeoff,
        )
    }
}

pub fn GetPlayerCustomSkin(playerid: isize) -> isize {
    unsafe { OMPRS_GetPlayerCustomSkin.unwrap()(playerid) }
}

pub fn RedirectDownload(playerid: isize, url: &str) -> bool {
    unsafe { OMPRS_RedirectDownload.unwrap()(playerid, cstr!(url)) }
}

pub fn FindModelFileNameFromCRC(crc: isize, output: &mut String, size: usize) -> bool {
    let mut addr = vec![0 as c_char; size];
    let ret = unsafe { OMPRS_FindModelFileNameFromCRC.unwrap()(crc, addr.as_mut_ptr()) };
    *output = from_cstr!(addr);
    ret
}

pub fn FindTextureFileNameFromCRC(crc: isize, output: &mut String, size: usize) -> bool {
    FindModelFileNameFromCRC(crc, output, size)
}

pub fn IsValidCustomModel(modelid: isize) -> bool {
    unsafe { OMPRS_IsValidCustomModel.unwrap()(modelid) }
}

pub fn GetCustomModelPath(
    modelid: isize,
    dff_path: &mut String,
    dff_path_size: usize,
    txd_path: &mut String,
    txd_path_size: usize,
) -> bool {
    let mut dff_path_addr = vec![0 as c_char; dff_path_size];
    let mut txd_path_addr = vec![0 as c_char; txd_path_size];

    let ret = unsafe {
        OMPRS_GetCustomModelPath.unwrap()(
            modelid,
            dff_path_addr.as_mut_ptr(),
            txd_path_addr.as_mut_ptr(),
        )
    };

    *dff_path = from_cstr!(dff_path_addr);
    *txd_path = from_cstr!(txd_path_addr);

    ret
}
