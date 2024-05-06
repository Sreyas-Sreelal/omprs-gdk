pub mod events;
pub mod functions;

use crate::{players::WeaponSlots, types::vector::Vector3};
pub use functions::{
    load_functions, CreateClass, EditClassData, GetAvailableClasses, GetClassData,
};

/// Player Class Information
#[repr(C)]
#[derive(Default)]
pub struct PlayerClass {
    /// The class's team
    team: u8,
    /// The class's skin ID            
    skin: isize,
    /// The class's spawn position         
    spawn: Vector3,
    /// The class's angle       
    angle: f32,
    /// The class's weapons          
    weapons: WeaponSlots,
}
