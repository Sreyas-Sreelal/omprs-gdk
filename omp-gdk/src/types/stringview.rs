#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct StringView {
    size: usize,
    data: *const std::ffi::c_char,
}

impl Default for StringView {
    fn default() -> Self {
        Self::new(std::ptr::null(), 0)
    }
}

impl StringView {
    pub fn new(data: *const std::ffi::c_char, size: usize) -> Self {
        StringView { data, size }
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
