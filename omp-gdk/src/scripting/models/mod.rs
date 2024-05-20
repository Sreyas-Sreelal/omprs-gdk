pub mod events;
pub mod functions;

pub use functions::load_functions;

/// Model Download Type
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ModelDownloadType {
    /// None
    NONE = 0,
    /// .dff file
    DFF = 1,
    /// .txd file
    TXD = 2,
}
