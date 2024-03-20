#[repr(C)]
pub struct StringView {
    data: *const std::ffi::c_char,
    size: usize,
}

impl Default for StringView {
    fn default() -> Self {
        Self::new()
    }
}

impl StringView {
    pub fn new() -> Self {
        StringView {
            data: std::ptr::null(),
            size: 0,
        }
    }
    pub fn get_data(&self) -> String {
        unsafe {
            std::ffi::CStr::from_ptr(self.data)
                .to_str()
                .unwrap()
                .to_owned()
        }
    }
}

impl From<&str> for StringView {
    fn from(value: &str) -> Self {
        StringView {
            data: value.as_ptr().cast(),
            size: value.len(),
        }
    }
}
