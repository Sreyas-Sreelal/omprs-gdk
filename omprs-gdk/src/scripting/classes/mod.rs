pub mod events;
pub mod functions;

use crate::{players::WeaponSlots, vector::Vector3};
pub use functions::{
    load_functions, CreateClass, EditClassData, GetAvailableClasses, GetClassData,
};

#[repr(C)]
#[derive(Default)]
pub struct PlayerClass {
    team: u8,             // The class's team
    skin: isize,          // The class's skin ID
    spawn: Vector3,       // The class's spawn position
    angle: f32,           // The class's angle
    weapons: WeaponSlots, // The class's weapons
}
