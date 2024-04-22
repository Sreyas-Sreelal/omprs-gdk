pub mod events;
pub mod functions;

pub use functions::load_functions;

#[repr(C)]
pub enum ModelDownloadType {
    NONE = 0,
    DFF = 1,
    TXD = 2,
}
