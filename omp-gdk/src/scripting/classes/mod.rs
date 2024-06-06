pub mod events;
pub mod functions;

use crate::{players::WeaponSlots, types::vector::Vector3};
pub use functions::{
    load_functions, CreateClass, EditClassData, GetAvailableClasses, GetClassData,
};

/// Player Class Information
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerClass {
    /// The class's team
    pub team: u8,
    /// The class's skin ID            
    pub skin: isize,
    /// The class's spawn position         
    pub spawn: Vector3,
    /// The class's angle       
    pub angle: f32,
    /// The class's weapons          
    pub weapons: WeaponSlots,
}
