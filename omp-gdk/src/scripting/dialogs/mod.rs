pub mod events;
pub mod functions;

pub use functions::load_functions;

/// Type of Dialog Styles
/// https://www.open.mp/docs/scripting/resources/dialogstyles
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

/// Type of Dialog Response
#[repr(C)]
#[derive(PartialEq)]
pub enum DialogResponse {
    /// right button
    Right = 0,
    /// left button
    Left,
}
