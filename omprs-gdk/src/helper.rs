use std::{ffi::CString, iter};

// TODO: write Linux code too
use windows::{
    core::{PCSTR, PCWSTR},
    Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
};

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
    unsafe {
        let handle = GetModuleHandleW(PCWSTR(module.as_ptr() as _)).unwrap();
        GetProcAddress(handle, PCSTR(symbol.as_ptr() as _)).map(|func| func as usize)
    }
}
