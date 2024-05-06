pub mod events;
pub mod functions;

pub use functions::load_functions;

#[repr(C)]
#[derive(PartialEq)]
pub enum DialogStyle {
    MsgBox = 0,
    Input,
    List,
    Password,
    TabList,
    TablistHeaders,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum DialogResponse {
    Right = 0,
    Left,
}
