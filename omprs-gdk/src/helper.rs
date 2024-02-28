use std::{ffi::CString, iter};

use windows::{
    core::{PCSTR, PCWSTR},
    Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
};

#[cfg(target_os = "linux")]
use std::os::raw;

#[cfg(target_os = "linux")]
#[cfg(link(name = "dl"))]
extern "C" {
    fn dlopen(filename: *const raw::c_char, flags: raw::c_int) -> *mut raw::c_void;
    fn dlsym(handle: *mut raw::c_void, symbol: *const raw::c_char) -> *mut raw::c_void;
    fn dlerror() -> *mut raw::c_char;
}

macro_rules! cstr {
    ($e: expr) => {{
        std::ffi::CString::new($e).unwrap().into_raw()
    }};
}

macro_rules! load_function {
    ($name: expr) => {
        paste! {
            let prefixed =  format!("OMPRS_{}",stringify!($name));
            let address = get_module_symbol_address("omprs", &prefixed)
                .expect(&format!("could not find '{prefixed}' address"));
            unsafe {
                [<OMPRS_ $name>] = Some(mem::transmute(address));
            }
        }
    };
}

pub fn get_module_symbol_address(module: &str, symbol: &str) -> Option<usize> {
    let module = module
        .encode_utf16()
        .chain(iter::once(0))
        .collect::<Vec<u16>>();
    let symbol = CString::new(symbol).unwrap();

    #[cfg(target_os = "windows")]
    unsafe {
        let handle = GetModuleHandleW(PCWSTR(module.as_ptr() as _)).unwrap();
        GetProcAddress(handle, PCSTR(symbol.as_ptr() as _)).map(|func| func as usize)
    }

    #[cfg(target_os = "linux")]
    unsafe {
        let handle = dlopen("omprs",1);
        dlsym(handle, symbol.as_ptr() as _) as usize
    }
}
