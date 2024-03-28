use std::ffi::{c_char, CStr};
use std::fmt;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct HybridString<const SIZE: usize> {
    size: usize,
    staticStorage: [c_char; SIZE],
}

impl<const SIZE: usize> fmt::Display for HybridString<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", unsafe {
            CStr::from_ptr(self.staticStorage.as_ptr())
                .to_string_lossy()
                .to_string()
        })
    }
}

impl<const SIZE: usize> From<&str> for HybridString<SIZE> {
    fn from(value: &str) -> Self {
        assert!(value.len() < SIZE);
        let mut staticStorage = [0; SIZE];
        for (idx, chara) in value.char_indices() {
            staticStorage[idx] = chara as c_char;
        }
        HybridString {
            size: value.len() << 1,
            staticStorage,
        }
    }
}
