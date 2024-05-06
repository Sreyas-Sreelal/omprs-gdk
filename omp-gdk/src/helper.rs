#[cfg(target_os = "windows")]
use windows::{
    core::{PCSTR, PCWSTR},
    Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
};

#[cfg(target_os = "linux")]
use std::os::raw;

#[cfg(target_os = "linux")]
#[link(name = "dl")]
extern "C" {
    fn dlsym(handle: *mut raw::c_void, symbol: *const raw::c_char) -> *mut raw::c_void;
}

macro_rules! load_function {
    ($name: expr) => {
        paste::paste! {
            let prefixed =  format!("OMPRS_{}",stringify!($name));
            let address = crate::helper::get_module_symbol_address("Rust", &prefixed)
                .expect(&format!("could not find '{prefixed}' address"));
            unsafe {
                [<OMPRS_ $name>] = Some(std::mem::transmute(address));
            }
        }
    };
}

#[cfg(target_os = "windows")]
pub fn get_module_symbol_address(module: &str, symbol: &str) -> Option<usize> {
    use std::{ffi::CString, iter};

    let module = module
        .encode_utf16()
        .chain(iter::once(0))
        .collect::<Vec<u16>>();
    let symbol = CString::new(symbol).unwrap();

    unsafe {
        let handle = GetModuleHandleW(PCWSTR(module.as_ptr() as _)).unwrap();
        GetProcAddress(handle, PCSTR(symbol.as_ptr() as _)).map(|func| func as usize)
    }
}

#[cfg(target_os = "linux")]
pub fn get_module_symbol_address(_module: &str, symbol: &str) -> Option<usize> {
    use std::ffi::CString;
    let symbol = CString::new(symbol).unwrap();

    unsafe { Some(dlsym(std::ptr::null_mut(), symbol.as_ptr()) as usize) }
}
