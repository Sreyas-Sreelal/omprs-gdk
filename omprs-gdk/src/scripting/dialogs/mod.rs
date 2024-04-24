pub mod events;
pub mod functions;

pub use functions::load_functions;

#[repr(C)]
pub enum DialogStyle {
    MsgBox = 0,
    Input,
    List,
    Password,
    TabList,
    TablistHeaders,
}

#[repr(C)]
pub enum DialogResponse {
    Right = 0,
    Left,
}
