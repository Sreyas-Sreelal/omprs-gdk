use std::ffi::c_char;
pub static mut OMPRS_Print: Option<unsafe extern "C" fn(*const c_char)> = None;

pub fn Console_Print(message: &str) {
    unsafe { OMPRS_Print.unwrap()(cstr!(message)) }
}
